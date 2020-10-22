//! This module defines the sender of a SPMC channel.

use crate::{
    error::NoReceivers,
    spmc::shared::{
        Node,
        NodeData,
        NodeDataPtr,
        OrphanData,
        SenderSubs,
        Shared,
        SubsData,
    },
};
use std::{
    fmt,
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
        let node_data = self.clear_unused_nodes();
        if !node_data.connected {
            // Safe because there are no receivers connected.
            let unreceived = unsafe { self.take_unreceived(node_data.ptr) };
            Err(NoReceivers { attempt: message, unreceived })?
        } else if node_data.ptr.is_null() {
            let msg_alloc = Self::make_orphan(message);

            // Safe because we won't use msg_alloc in case of error,
            // while prev_msg_ptr is always discarded. Also,
            // prev_msg_ptr was acquired from the front. Msg_alloc was
            // allocated by a Box, and the box is leaked.
            let res = unsafe { self.send_through_orphan(None, msg_alloc) };
            match res {
                Ok(unreceived) => Ok(unreceived),

                // Safe because Msg_alloc was allocated by a leaked Box,
                // and data_ptr was acquired from the front's first
                // node's data. Also, we use neither of these pointers
                // after we call the function.
                Err(node_data) => unsafe {
                    self.send_loop(node_data, msg_alloc)
                },
            }
        } else {
            match node_data.ptr {
                NodeDataPtr::Orphan(prev_msg_ptr) => {
                    let msg_alloc = Self::make_orphan(message);
                    let prev_nonnull = NonNull::new(prev_msg_ptr);

                    // Safe because we won't use msg_alloc in case of error,
                    // while prev_msg_ptr is always discarded. Also,
                    // prev_msg_ptr was acquired from the front. Msg_alloc was
                    // allocated by a Box, and the box is leaked.
                    let res = unsafe {
                        self.send_through_orphan(prev_nonnull, msg_alloc)
                    };
                    match res {
                        Ok(unreceived) => Ok(unreceived),

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
        let null = NodeDataPtr::<T>::null();
        let node_data = NodeData { ptr: null, connected: true };
        self.front_ref().data.store(node_data.encode(), Release);
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
        expected: Option<NonNull<OrphanData<T>>>,
        msg_alloc: NonNull<OrphanData<T>>,
    ) -> Result<Option<T>, NodeData<T>> {
        let node_data_ptr = expected.map_or(NodeDataPtr::null(), |nonnull| {
            NodeDataPtr::Orphan(nonnull.as_ptr())
        });
        let node_data = NodeData { ptr: node_data_ptr, connected: true };
        let new_ptr = NodeDataPtr::Orphan(msg_alloc.as_ptr());
        let new_node_data = NodeData { ptr: new_ptr, connected: true };
        let res = self.front_ref().data.compare_exchange(
            node_data.encode(),
            new_node_data.encode(),
            AcqRel,
            Relaxed,
        );

        match res {
            Ok(_) => Ok(expected.map(|nonnull| {
                let boxed = Box::from_raw(nonnull.as_ptr());
                boxed.message
            })),
            Err(bits) => Err(NodeData::decode(bits)),
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
        mut node_data: NodeData<T>,
        msg_alloc: NonNull<OrphanData<T>>,
    ) -> Result<Option<T>, NoReceivers<T>> {
        loop {
            if !node_data.connected {
                let msg_box = Box::from_raw(msg_alloc.as_ptr());
                let message = msg_box.message;
                let unreceived = self.take_unreceived(node_data.ptr);
                Err(NoReceivers { attempt: message, unreceived })?
            } else if node_data.ptr.is_null() {
                let res = self.send_through_orphan(None, msg_alloc);
                match res {
                    Ok(nothing) => break Ok(nothing),
                    Err(update) => node_data = update,
                }
            } else {
                match node_data.ptr {
                    NodeDataPtr::Orphan(prev_msg) => {
                        let res = self.send_through_orphan(
                            NonNull::new(prev_msg),
                            msg_alloc,
                        );
                        match res {
                            Ok(unreceived) => break Ok(unreceived),
                            Err(update) => node_data = update,
                        }
                    },

                    NodeDataPtr::Subs(subs_data) => {
                        let msg_box = Box::from_raw(msg_alloc.as_ptr());
                        let message = msg_box.message;
                        self.send_through_subs(subs_data, message);
                        break Ok(None);
                    },
                }
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
    /// `data_ptr` must have been loaded from the subscription queue's front.
    /// All receivers must have disconnected before loading `data_ptr`.
    unsafe fn take_unreceived(
        &mut self,
        data_ptr: NodeDataPtr<T>,
    ) -> Option<T> {
        if data_ptr.is_null() {
            None
        } else {
            match data_ptr {
                NodeDataPtr::Subs(_) => None,
                NodeDataPtr::Orphan(ptr) => {
                    let null = NodeDataPtr::<T>::null();
                    let node_data = NodeData { ptr: null, connected: false };
                    *self.front_mut().data.get_mut() = node_data.encode();
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
    fn clear_unused_nodes(&mut self) -> NodeData<T> {
        loop {
            let front = self.front_ref();
            let node_data = NodeData::<T>::decode(front.data.load(Acquire));

            if !node_data.ptr.is_null() {
                break node_data;
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
                None => break node_data,
            }
        }
    }

    fn make_orphan(message: T) -> NonNull<OrphanData<T>> {
        let msg_box = Box::new(OrphanData { message });
        NonNull::from(Box::leak(msg_box))
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Sender")
            .field("shared", &self.shared)
            .field("front", &self.front)
            .finish()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {}
}
