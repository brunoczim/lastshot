//! This module defines the receiver of a SPMC channel.

use crate::{
    error::NoSenders,
    spmc::{
        node::{
            Node,
            NodeData,
            NodeDataPtr,
            OrphanData,
            ReceiverSubs,
            SenderSubs,
            SubsRecv,
        },
        shared::Shared,
    },
};
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    ptr::NonNull,
    sync::{
        atomic::{AtomicPtr, AtomicUsize, Ordering::*},
        Arc,
    },
    task,
};

/// The receiver handle of a SPMC last-shot channel. Since the channel is a
/// "multi consumer" channel (spMC), it is possible to clone this handle, but
/// it requires a mutable reference. The cost of cloning involves incrementing
/// an atomic variable and making a new small allocation.
pub struct Receiver<T> {
    /// Structure shared with receiver.
    shared: Arc<Shared<T>>,
    /// The receiver handle to the subscription data it owns. To subscribe for
    /// a new message, it uses this object.
    subs: ReceiverSubs<T>,
}

impl<T> Receiver<T> {
    /// Creates a new sender from the given shared structure ARC.
    ///
    /// # Safety
    /// Safe if the shared structure can only be shared with the sender, and it
    /// must control its presence flag correctly (while active set to 1).
    pub(crate) unsafe fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared, subs: ReceiverSubs::new() }
    }

    pub fn try_recv(&self) -> Result<Option<T>, NoSenders> {
        let dummy = Self::make_dummy_node();
        let back_ptr = self.shared.back().swap(dummy.as_ptr(), AcqRel);
        let back = unsafe { NonNull::new_unchecked(back_ptr) };

        let bits = unsafe { back.as_ref().data.load(Acquire) };
        let node_data = NodeData::decode(bits);

        let result = match node_data.ptr {
            NodeDataPtr::Orphan(orphan) if !orphan.is_null() => {
                let orphan = unsafe { NonNull::new_unchecked(orphan) };
                let message =
                    unsafe { self.take_orphan(orphan, node_data, back) };
                Ok(Some(message))
            },

            _ => {
                if node_data.connected {
                    Ok(None)
                } else {
                    Err(NoSenders)
                }
            },
        };

        unsafe { self.try_rollback(dummy, back) }

        result
    }

    pub async fn recv(&mut self) -> Result<T, NoSenders> {
        Subscriber::new(self).await
    }

    unsafe fn try_rollback(
        &self,
        expected: NonNull<Node<T>>,
        prev_back: NonNull<Node<T>>,
    ) {
        let res = self.shared.back().compare_exchange(
            expected.as_ptr(),
            prev_back.as_ptr(),
            Release,
            Relaxed,
        );

        match res {
            Ok(_) => {
                Box::from_raw(expected.as_ptr());
            },

            Err(_) => prev_back.as_ref().next.store(expected.as_ptr(), Release),
        }
    }

    unsafe fn take_orphan(
        &self,
        orphan: NonNull<OrphanData<T>>,
        node_data: NodeData<T>,
        prev_back: NonNull<Node<T>>,
    ) -> T {
        let mut new_node_data = NodeData {
            connected: node_data.connected,
            ptr: NodeDataPtr::<T>::null(),
        };
        let res = prev_back.as_ref().data.compare_exchange(
            node_data.encode(),
            new_node_data.encode(),
            Release,
            Relaxed,
        );
        if res.is_err() {
            new_node_data.connected = false;
            prev_back.as_ref().data.store(new_node_data.encode(), Release);
        }

        Box::from_raw(orphan.as_ptr()).message
    }

    fn make_dummy_node() -> NonNull<Node<T>> {
        let null = NodeDataPtr::<T>::null();
        let node_data = NodeData { ptr: null, connected: true };

        let node_box = Box::new(Node {
            data: AtomicUsize::new(node_data.encode()),
            next: AtomicPtr::default(),
            _marker: PhantomData,
        });

        NonNull::from(Box::leak(node_box))
    }
}

unsafe impl<T> Send for Receiver<T> where T: Send {}
unsafe impl<T> Sync for Receiver<T> where T: Send {}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.shared.create_receiver();
        Self { shared: self.shared.clone(), subs: ReceiverSubs::new() }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let connection = unsafe { self.shared.drop_receiver() };
        if connection.receivers == Some(0) {
            let replacement =
                NodeData { connected: false, ptr: NodeDataPtr::<T>::null() };
            let back_ptr = self.shared.back().load(Acquire);
            let bits =
                unsafe { (*back_ptr).data.swap(replacement.encode(), AcqRel) };
            let node_data = NodeData::<T>::decode(bits);
            match node_data.ptr {
                NodeDataPtr::Subs(ptr) => {
                    if let Some(ptr) = NonNull::new(ptr) {
                        unsafe {
                            SenderSubs::from_raw(ptr);
                        }
                    }
                },

                NodeDataPtr::Orphan(ptr) => {
                    if !ptr.is_null() {
                        unsafe { Box::from_raw(ptr) };
                    }
                },
            }

            let connection = unsafe { self.shared.drop_receivers() };

            if !connection.sender {
                unsafe {
                    Box::from_raw(back_ptr);
                }
            }
        }
    }
}

struct Subscriber<'receiver, T> {
    receiver: &'receiver mut Receiver<T>,
    node: NonNull<Node<T>>,
    prev_back: NonNull<Node<T>>,
    done: bool,
}

impl<'receiver, T> Subscriber<'receiver, T> {
    fn new(receiver: &'receiver mut Receiver<T>) -> Self {
        let node = Receiver::make_dummy_node();
        let back = receiver.shared.back().swap(node.as_ptr(), AcqRel);

        Self {
            receiver,
            node,
            prev_back: unsafe { NonNull::new_unchecked(back) },
            done: false,
        }
    }

    unsafe fn post_subscribed(&mut self) -> task::Poll<Result<T, NoSenders>> {
        let sender_subs = self.receiver.subs.sender_subs();
        let raw_subs = SenderSubs::into_raw(sender_subs);
        let expected =
            NodeData { ptr: NodeDataPtr::<T>::null(), connected: true };
        let new_node_data = NodeData {
            ptr: NodeDataPtr::Subs(raw_subs.as_ptr()),
            connected: true,
        };
        let res = self.prev_back.as_ref().data.compare_exchange(
            expected.encode(),
            new_node_data.encode(),
            AcqRel,
            Acquire,
        );

        match res {
            Ok(_) => {
                self.prev_back.as_ref().next.store(self.node.as_ptr(), Release);
                task::Poll::Pending
            },

            Err(bits) => {
                let node_data = NodeData::<T>::decode(bits);
                SenderSubs::from_raw(raw_subs);
                self.receiver.subs.cancel_subs();
                match node_data.ptr {
                    NodeDataPtr::Orphan(ptr) if !ptr.is_null() => {
                        let orphan = NonNull::new_unchecked(ptr);
                        let message = self.receiver.take_orphan(
                            orphan,
                            node_data,
                            self.prev_back,
                        );
                        self.receiver.try_rollback(self.node, self.prev_back);
                        task::Poll::Ready(Ok(message))
                    },
                    _ => task::Poll::Ready(Err(NoSenders)),
                }
            },
        }
    }
}

unsafe impl<'receiver, T> Send for Subscriber<'receiver, T> where T: Send {}
unsafe impl<'receiver, T> Sync for Subscriber<'receiver, T> where T: Send {}

impl<'receiver, T> Future for Subscriber<'receiver, T> {
    type Output = Result<T, NoSenders>;

    fn poll(
        mut self: Pin<&mut Self>,
        ctx: &mut task::Context,
    ) -> task::Poll<Self::Output> {
        if self.done {
            panic!("spmc::receiver::Subscribe polled after done")
        }

        let poll = match self.receiver.subs.subscribe_or_recv(ctx.waker()) {
            SubsRecv::Subscribed => unsafe { self.post_subscribed() },
            SubsRecv::Waiting => task::Poll::Pending,
            SubsRecv::NoSender => task::Poll::Ready(Err(NoSenders)),
            SubsRecv::Received(message) => task::Poll::Ready(Ok(message)),
        };

        self.done = poll.is_ready();

        poll
    }
}
