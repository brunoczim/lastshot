//! This module defines the sender of a SPMC channel.

use crate::{
    error::NoReceivers,
    spmc::shared::{
        Node,
        NodeDataPtr,
        OrphanData,
        SenderSubs,
        Shared,
        SubsData,
    },
};
use std::{
    ptr::NonNull,
    sync::{atomic::Ordering::*, Arc},
};

/// The sender handle of a SPMC last-shot channel. Since the channel is a
/// "single producer" channel (SPmc), it is not possible to clone this handle,
/// and it requires a mutable reference.
pub struct Sender<T> {
    /// Structure shared with receiver.
    shared: Arc<Shared<T>>,
    /// A pointer to the front of the subscription queue.
    front: NonNull<Node<T>>,
}

impl<T> Sender<T> {
    /// Creates a new sender from the given shared structure ARC.
    ///
    /// # Safety
    /// Safe if the single_node parameter is present as the back of the
    /// subscription queue in the shared structure, if it has both `data` and
    /// `next` set to `null`, and it is not shared anywhere else. The shared
    /// structure can only be shared with receivers, and they must control
    /// receivers count correctly (each active receiver handle counts as 1).
    pub(crate) unsafe fn new(
        shared: Arc<Shared<T>>,
        single_node: NonNull<Node<T>>,
    ) -> Self {
        Self { shared, front: single_node }
    }

    /// Sends a message through the channel. Sending a message always overwrite
    /// the previous message (if not received).
    ///
    /// If all [`Receiver`](crate::mpsc::Receiver)s disconnected, it
    /// returns `Err(NoReceivers)`, and the error will contain:
    /// - The parameter `message` that the caller attempted to send.
    /// - The value stored in the channel that has never been received, if any.
    ///
    /// Otherwise, if there are receivers, it returns the previous message if
    /// never received, wrapped inside `Ok(Some(message))`.
    pub fn send(&mut self, message: T) -> Result<Option<T>, NoReceivers<T>> {
        if self.shared.receivers() == 0 {
            // Safe because there are no receivers connected.
            let unreceived = unsafe { self.take_unreceived() };
            Err(NoReceivers { attempt: message, unreceived })?
        } else {
            match self.clear_unused_nodes() {
                NodeDataPtr::Orphan(prev_msg_ptr) => {
                    let msg_box = Box::new(OrphanData { message });
                    let msg_alloc = NonNull::from(Box::leak(msg_box));

                    // Safe because we won't use msg_alloc in case of error,
                    // while prev_msg_ptr is always discarded. Also,
                    // prev_msg_ptr was acquired from the front. Msg_alloc was
                    // allocated by a Box, and the box is leaked.
                    let res = unsafe {
                        self.send_through_orphan(prev_msg_ptr, msg_alloc)
                    };
                    match res {
                        Ok(unreceived) => Ok(Some(unreceived)),

                        // Safe because Msg_alloc was allocated by a leaked Box,
                        // and data_ptr was acquired from the front's first
                        // node's data. Also, we use neither of these pointers
                        // after we call the function.
                        Err(data_ptr) => unsafe {
                            self.send_loop(data_ptr, msg_alloc)
                        },
                    }
                },

                NodeDataPtr::Subs(ptr) => {
                    // Safe because we got this pointer from front's first node
                    // data. Also, we did not reconstruct the `SenderSubs`
                    // struct.
                    unsafe { self.send_through_subs(ptr, message) }
                    Ok(None)
                },
            }
        }
    }

    /// Sends a message using a subscriber data pointer.
    ///
    /// # Safety
    /// The subscriber data pointer must have been acquired by
    /// [`SenderSubs::into_raw`] (i.e. must be a valid input to
    /// [`SenderSubs::from_raw`] and must not be null). It must not have been
    /// used to reconstruct a [`SenderSubs`] after the last call to
    /// [`SenderSubs::into_raw`], and must not be used after this function
    /// is executed since the [`SenderSubs`] is destroyed.
    unsafe fn send_through_subs(
        &mut self,
        subs_data: *mut SubsData<T>,
        message: T,
    ) {
        self.front_ref().data.store(NodeDataPtr::<T>::null().encode(), Release);
        let nonnull = NonNull::new_unchecked(subs_data);
        SenderSubs::from_raw(nonnull).send(message);
    }

    /// Sends a message using an orphan node's data pointer. Returns `Ok` if the
    /// message is sent successfully, wrapping the previous message. Otherwise,
    /// it returns an error containing the new data pointer stored in the first
    /// node's data field.
    ///
    /// # Safety
    /// `expected` must have been loaded from the front's first node data field.
    /// `msg_alloc` must point to a valid address, allocated by a (leaked)
    /// `Box`, and must be initialized to a message. Neither of these pointers
    /// should be used by the sender after this function is called.
    unsafe fn send_through_orphan(
        &mut self,
        expected: *mut OrphanData<T>,
        msg_alloc: NonNull<OrphanData<T>>,
    ) -> Result<T, NodeDataPtr<T>> {
        let new_data_ptr = NodeDataPtr::Orphan(msg_alloc.as_ptr());
        let res = self.front_ref().data.compare_exchange(
            NodeDataPtr::Orphan(expected).encode(),
            new_data_ptr.encode(),
            AcqRel,
            Relaxed,
        );

        match res {
            Ok(_) => {
                let boxed = Box::from_raw(expected);
                Ok(boxed.message)
            },
            Err(bits) => Err(NodeDataPtr::decode(bits)),
        }
    }

    /// Sends a message using a send loop, which continuously attempt to send a
    /// message through either a subscriber node or an orphan node. It also
    /// checks for the connection of receivers. Returns `Ok` wrapping the
    /// previous message (if unreceived) in case of success. Returns error
    /// if all receivers disconnect.
    ///
    /// # Safety
    /// `data_ptr` must have been loaded from the front's first node data field.
    /// `msg_alloc` must point to a valid address, allocated by a (leaked)
    /// `Box`, and must be initialized to a message. Neither of these pointers
    /// should be used by the sender after this function is called.
    unsafe fn send_loop(
        &mut self,
        mut data_ptr: NodeDataPtr<T>,
        msg_alloc: NonNull<OrphanData<T>>,
    ) -> Result<Option<T>, NoReceivers<T>> {
        loop {
            match data_ptr {
                NodeDataPtr::Subs(subs_data) => {
                    let msg_box = Box::from_raw(msg_alloc.as_ptr());
                    let message = msg_box.message;
                    self.send_through_subs(subs_data, message);
                    break Ok(None);
                },

                NodeDataPtr::Orphan(prev_msg_ptr) => {
                    let res = self.send_through_orphan(prev_msg_ptr, msg_alloc);
                    match res {
                        Ok(prev_msg) => break Ok(Some(prev_msg)),
                        Err(update) => data_ptr = update,
                    }
                },
            }

            if self.shared.receivers() == 0 {
                let msg_box = Box::from_raw(msg_alloc.as_ptr());
                let message = msg_box.message;
                let unreceived = self.take_unreceived();
                Err(NoReceivers { attempt: message, unreceived })?
            }
        }
    }

    /// Returns an immutable reference to the front's first node.
    fn front_ref(&self) -> &Node<T> {
        // Safe because we require the necessary invariants in the constructor.
        // We are the only ones with power to drop the front.
        unsafe { self.front.as_ref() }
    }

    /// Returns a mutable reference to the front's first node.
    ///
    /// # Safety
    /// All receivers must have disconnected.
    unsafe fn front_mut(&mut self) -> &mut Node<T> {
        self.front.as_mut()
    }

    /// Returns an unreceived message, if there is one.
    ///
    /// # Safety
    /// All receivers must have disconnected.
    unsafe fn take_unreceived(&mut self) -> Option<T> {
        let data_ptr = self.clear_unused_nodes();

        if data_ptr.is_null() {
            None
        } else {
            match data_ptr {
                NodeDataPtr::Subs(_) => None,
                NodeDataPtr::Orphan(ptr) => {
                    let null = NodeDataPtr::<T>::null().encode();
                    *self.front_mut().data.get_mut() = null;
                    Some(Box::from_raw(ptr).message)
                },
            }
        }
    }

    /// Clears nodes that are "unused" until an "used" node is found. An used
    /// node is a node that either:
    ///
    /// - Has the data field set to something different than null.
    /// - Or it is the last node.
    ///
    /// Otherwise, it is unused.
    fn clear_unused_nodes(&mut self) -> NodeDataPtr<T> {
        loop {
            let front = self.front_ref();
            let data_ptr = NodeDataPtr::<T>::decode(front.data.load(Acquire));

            if !data_ptr.is_null() {
                break data_ptr;
            }

            let maybe_next = NonNull::new(front.next.load(Acquire));
            match maybe_next {
                Some(next) => {
                    unsafe {
                        // Safe because we guarantee through the constructor we
                        // are the only ones with access to it.
                        Box::from_raw(self.front.as_ptr());
                    }
                    self.front = next;
                },
                None => break data_ptr,
            }
        }
    }
}
