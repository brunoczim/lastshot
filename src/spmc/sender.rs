//! This module defines the sender of a SPMC channel.

use crate::{
    error::NoReceivers,
    spmc::{
        node::{Node, NodeData, NodeDataPtr, OrphanData, SenderSubs, SubsData},
        shared::Shared,
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
        // We have a contract that says the pointer cannot be null.
        let nonnull = NonNull::new_unchecked(subs_data);
        // Ok, the contract require the pointer to be valid.
        SenderSubs::from_raw(nonnull).send(message);
    }

    /// Sends a message using an orphan node's data pointer. Returns `Ok` if the
    /// message is sent successfully, wrapping the previous message, and
    /// `msg_alloc` is not usable anymore by the caller. Otherwise, it returns
    /// an error containing the new data pointer stored in the first
    /// node's data field and `msg_alloc` is usable by the caller.
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
            Ok(_) => match expected {
                Some(nonnull) => {
                    // Checked for null, and the contract requires the pointer
                    // to be loaded from the front, and so
                    // it would follow our invariants.
                    let boxed = Box::from_raw(nonnull.as_ptr());
                    Ok(Some(boxed.message))
                },

                None => {
                    if self.front_ref().next.load(Acquire).is_null() {
                        Ok(None)
                    } else {
                        let empty_node_data = NodeData {
                            connected: true,
                            ptr: NodeDataPtr::<T>::null(),
                        };
                        let res = self.front_ref().data.compare_exchange(
                            new_node_data.encode(),
                            empty_node_data.encode(),
                            AcqRel,
                            Release,
                        );

                        match res {
                            Ok(_) => Err(self.clear_unused_nodes()),

                            Err(_) => Ok(None),
                        }
                    }
                },
            },
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
                // Ok, msg_alloc is only ever used if no operation with it
                // succeed, and we obtain it from a Box, it should be valid.
                let msg_box = Box::from_raw(msg_alloc.as_ptr());
                let message = msg_box.message;
                let unreceived = self.take_unreceived(node_data.ptr);
                Err(NoReceivers { attempt: message, unreceived })?
            } else if node_data.ptr.is_null() {
                // Again, msg_alloc is only ever used if no operation with it
                // succeed, and we obtain it from a Box, it should be valid.
                let res = self.send_through_orphan(None, msg_alloc);
                match res {
                    Ok(nothing) => {
                        if self.front_ref().next.load(Relaxed).is_null() {
                            break Ok(nothing);
                        }
                        node_data = self.clear_unused_nodes();
                    },
                    Err(update) => node_data = update,
                }
            } else {
                match node_data.ptr {
                    NodeDataPtr::Orphan(prev_msg) => {
                        // We require msg_alloc to be valid, and we only use it
                        // after this if the result is an error.
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
                        // Ok, msg_alloc must be valid since we need it for
                        // other calls, such as send_through_orphan (possibly
                        // called previously).
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
                    // We require ptr to be valid through the contract.
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
                        // are the only ones with access to it. Front being a
                        // valid pointer is a basic invariant needed by the
                        // sender.
                        Box::from_raw(self.front.as_ptr());
                    }
                    self.front = next;
                },
                None => break node_data,
            }
        }
    }

    /// Wraps a message in a heap-allocation to an orphan node.
    fn make_orphan(message: T) -> NonNull<OrphanData<T>> {
        let msg_box = Box::new(OrphanData { message });
        NonNull::from(Box::leak(msg_box))
    }

    /// Warns the receivers that the sender is dropping through a node.
    /// Essentially, it turns of the connected bit.
    ///
    /// # Safety
    /// Safe only if called in the destructor. More specifically, the sender
    /// must not be used after this function, except for cleanup. NodeData
    /// pointer must have been obtained from a node, and must be usable if
    /// we successefully replace it with a compare_exchange operation.
    unsafe fn warn_on_node(
        &mut self,
        node_data: NodeData<T>,
    ) -> Result<(), NodeData<T>> {
        let mut new_node_data =
            NodeData { connected: false, ptr: node_data.ptr };

        if let NodeDataPtr::Subs(subs_ptr) = node_data.ptr {
            new_node_data.ptr = NodeDataPtr::null();
            if let Some(nonnull) = NonNull::new(subs_ptr) {
                SenderSubs::from_raw(nonnull);
            }
        }

        let res = self.front_ref().data.compare_exchange(
            node_data.encode(),
            new_node_data.encode(),
            AcqRel,
            Acquire,
        );

        match res {
            Ok(_) => Ok(()),
            Err(update) => Err(NodeData::decode(update)),
        }
    }

    /// Clears a node which has been used to warn a receiver. Essentially, this
    /// takes the sender's handle to a subscriber, if the node data pointer
    /// points to subscrition data. Also, this will set the front to its next
    /// node, if any, and it will free the memory of the warned node.
    ///
    /// # Safety
    /// Safe only if called in the destructor. More specifically, the sender
    /// must not be used after this function, except for cleanup. NodeData
    /// pointer must have been obtained from a node, and must be owned by this
    /// function if it is a subscriber data pointer (i.e. subscriber taken from
    /// the queue).
    unsafe fn clear_warned_node(&mut self, node_data: NodeData<T>) -> bool {
        if let NodeDataPtr::Subs(subs_ptr) = node_data.ptr {
            if let Some(nonnull) = NonNull::new(subs_ptr) {
                // Ok, we require the node to be valid through the contract.
                SenderSubs::from_raw(nonnull);
            }
        }

        let ptr = self.front_ref().next.load(Acquire);
        let maybe_next = NonNull::new(ptr);
        match maybe_next {
            Some(next) => {
                // We guarantee by the constructor this is valid, and we
                // manipulate it so it still valid. Front being a valid pointer
                // is a basic invariant needed by the sender.
                Box::from_raw(self.front.as_ptr());
                self.front = next;
                true
            },
            None => false,
        }
    }

    /// Warn receivers that the sender is being dropped, through all necessary
    /// nodes.
    ///
    /// # Safety
    /// Safe only if called in the destructor. More specifically, the sender
    /// must not be used after this function, except for cleanup.
    unsafe fn warn_receivers(&mut self) {
        let bits = self.front_ref().data.load(Acquire);
        let mut node_data = NodeData::<T>::decode(bits);
        while !node_data.connected {
            let res = self.warn_on_node(node_data);

            match res {
                Ok(_) => {
                    if !self.clear_warned_node(node_data) {
                        break;
                    }
                },
                Err(update) => node_data = update,
            }
        }
    }

    /// Destroy everything in the channel.
    ///
    /// # Safety
    /// Must be called in drop, and only if the receivers have all already
    /// disconnected. The channel should not be touched at all, as well the
    /// sender's front pointer.
    unsafe fn destroy_everything(&mut self) {
        loop {
            let bits = self.front_ref().data.load(Acquire);
            let node_data = NodeData::<T>::decode(bits);
            match node_data.ptr {
                NodeDataPtr::Subs(subs_ptr) => {
                    if let Some(nonnull) = NonNull::new(subs_ptr) {
                        // This pointer being valid is a basic invariant. Also,
                        // no one is accessing it after us, we require this
                        // function to be the last thing executed by the
                        // channel.
                        SenderSubs::from_raw(nonnull);
                    }
                },

                NodeDataPtr::Orphan(orphan_ptr) => {
                    if !orphan_ptr.is_null() {
                        // Ok, we checked for null, and we are the last thing
                        // executed by the channel.
                        Box::from_raw(orphan_ptr);
                    }
                },
            }

            // We are the last operation. Front can be freely destroyed even if
            // this is the last node.
            let ptr = self.front_ref().next.load(Acquire);
            Box::from_raw(self.front.as_ptr());
            let maybe_next = NonNull::new(ptr);
            match maybe_next {
                Some(next) => self.front = next,
                None => break,
            }
        }
    }
}

unsafe impl<T> Send for Sender<T> where T: Send {}
unsafe impl<T> Sync for Sender<T> where T: Send {}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Sender")
            .field("shared", &self.shared)
            .field("front", &self.front)
            .finish()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Ok, we are on drop and we won't send anything through the channel.
        unsafe { self.warn_receivers() }
        // Ok, we are the only sender, signaling once the drop that is
        // happening.
        let connected = unsafe { self.shared.drop_sender() };
        if connected.receivers.is_none() {
            // Ok, we won't touch anything afterm and all receivers dropped.
            unsafe { self.destroy_everything() }
        }
    }
}
