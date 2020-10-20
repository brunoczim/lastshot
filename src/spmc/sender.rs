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
            let unreceived = unsafe { self.take_unreceived() };
            Err(NoReceivers { attempt: message, unreceived })?
        } else {
            match self.clear_unused_nodes() {
                NodeDataPtr::Orphan(prev_msg_ptr) => {
                    let msg_box = Box::new(OrphanData { message });
                    let msg_alloc = NonNull::from(Box::leak(msg_box));
                    let res = unsafe {
                        self.send_through_orphan(prev_msg_ptr, msg_alloc)
                    };
                    match res {
                        Ok(unreceived) => Ok(unreceived),
                        Err(data_ptr) => unsafe {
                            self.send_loop(data_ptr, msg_alloc)
                        },
                    }
                },

                NodeDataPtr::Subs(ptr) => {
                    unsafe { self.send_through_subs(ptr, message) }
                    Ok(None)
                },
            }
        }
    }

    unsafe fn send_through_subs(
        &mut self,
        subs_data: *mut SubsData<T>,
        message: T,
    ) {
        let nonnull = NonNull::new_unchecked(subs_data);
        SenderSubs::from_raw(nonnull).send(message);
    }

    unsafe fn send_through_orphan(
        &mut self,
        expected: *mut OrphanData<T>,
        msg_alloc: NonNull<OrphanData<T>>,
    ) -> Result<Option<T>, NodeDataPtr<T>> {
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
                Ok(Some(boxed.message))
            },
            Err(bits) => Err(NodeDataPtr::decode(bits)),
        }
    }

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
                        Ok(prev_msg) => break Ok(prev_msg),
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

    fn front_ref(&self) -> &Node<T> {
        unsafe { self.front.as_ref() }
    }

    unsafe fn front_mut(&mut self) -> &mut Node<T> {
        self.front.as_mut()
    }

    unsafe fn take_unreceived(&mut self) -> Option<T> {
        loop {
            let front = self.front_mut();
            let data_ptr = NodeDataPtr::<T>::decode(*front.data.get_mut());

            if !data_ptr.is_null() {
                match data_ptr {
                    NodeDataPtr::Subs(_) => break None,
                    NodeDataPtr::Orphan(ptr) => {
                        let null = NodeDataPtr::<T>::null().encode();
                        *front.data.get_mut() = null;
                        break Some(Box::from_raw(ptr).message);
                    },
                }
            }

            let maybe_next = NonNull::new(*front.next.get_mut());
            match maybe_next {
                Some(next) => {
                    Box::from_raw(self.front.as_ptr());
                    self.front = next;
                },
                None => break None,
            }
        }
    }

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
                        Box::from_raw(self.front.as_ptr());
                    }
                    self.front = next;
                },
                None => break data_ptr,
            }
        }
    }
}
