//! This module defines the receiver of a SPMC channel.

use crate::{
    error::NoSenders,
    spmc::{
        node::{
            Node,
            NodeData,
            NodeDataPtr,
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
        let back = self.shared.back().swap(dummy.as_ptr(), AcqRel);

        let bits = unsafe { (*back).data.load(Acquire) };
        let node_data = NodeData::decode(bits);

        let result = match node_data.ptr {
            NodeDataPtr::Orphan(orphan_ptr) if !orphan_ptr.is_null() => {
                let mut new_node_data = NodeData {
                    connected: node_data.connected,
                    ptr: NodeDataPtr::<T>::null(),
                };
                let res = unsafe {
                    (*back).data.compare_exchange(
                        bits,
                        new_node_data.encode(),
                        Release,
                        Relaxed,
                    )
                };
                if res.is_err() {
                    new_node_data.connected = false;
                    unsafe {
                        (*back).data.store(new_node_data.encode(), Release);
                    }
                }

                let orphan = unsafe { Box::from_raw(orphan_ptr) };
                Ok(Some(orphan.message))
            },

            _ => {
                if node_data.connected {
                    Ok(None)
                } else {
                    Err(NoSenders)
                }
            },
        };

        unsafe { self.try_rollback(dummy.as_ptr(), back) }

        result
    }

    pub async fn recv(&mut self) -> Result<T, NoSenders> {
        let dummy = Self::make_dummy_node();
        let back = self.shared.back().swap(dummy.as_ptr(), AcqRel);

        let subscriber = unsafe {
            Subscriber::new(self, dummy, NonNull::new_unchecked(back))
        };

        subscriber.await
    }

    unsafe fn try_rollback(
        &self,
        expected: *mut Node<T>,
        prev_back: *mut Node<T>,
    ) {
        let res = self
            .shared
            .back()
            .compare_exchange(expected, prev_back, Release, Relaxed);

        match res {
            Ok(_) => {
                Box::from_raw(expected);
            },

            Err(_) => (*prev_back).next.store(expected, Release),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SubsState {
    TakeOrphan,
    Subscribe,
    Done,
}

struct Subscriber<'receiver, T> {
    receiver: &'receiver mut Receiver<T>,
    node: NonNull<Node<T>>,
    prev_back: NonNull<Node<T>>,
    state: SubsState,
}

impl<'receiver, T> Subscriber<'receiver, T> {
    unsafe fn new(
        receiver: &'receiver mut Receiver<T>,
        node: NonNull<Node<T>>,
        prev_back: NonNull<Node<T>>,
    ) -> Self {
        Self { receiver, node, prev_back, state: SubsState::TakeOrphan }
    }

    unsafe fn take_orphan(&mut self) -> task::Poll<Result<T, NoSenders>> {
        let bits = self.prev_back.as_ref().data.load(Acquire);
        let node_data = NodeData::decode(bits);
        match node_data.ptr {
            NodeDataPtr::Orphan(orphan_ptr) if !orphan_ptr.is_null() => {
                let mut new_node_data = NodeData {
                    connected: node_data.connected,
                    ptr: NodeDataPtr::<T>::null(),
                };
                let res = self.prev_back.as_ref().data.compare_exchange(
                    bits,
                    new_node_data.encode(),
                    Release,
                    Relaxed,
                );
                if res.is_err() {
                    new_node_data.connected = false;
                    self.prev_back
                        .as_ref()
                        .data
                        .store(new_node_data.encode(), Release);
                }

                let orphan = Box::from_raw(orphan_ptr);
                task::Poll::Ready(Ok(orphan.message))
            },

            _ => {
                self.state = SubsState::Subscribe;
                task::Poll::Pending
            },
        }
    }

    unsafe fn subscribe(
        &mut self,
        waker: &task::Waker,
    ) -> task::Poll<Result<T, NoSenders>> {
        match self.receiver.subs.subscribe_or_recv(waker) {
            SubsRecv::Subscribed => {
                let sender_subs = self.receiver.subs.sender_subs();
                let node_data = NodeData {
                    ptr: NodeDataPtr::Subs(
                        SenderSubs::into_raw(sender_subs).as_ptr(),
                    ),
                    connected: true,
                };
                self.node.as_ref().data.store(node_data.encode(), Release);
                self.prev_back.as_ref().next.store(self.node.as_ptr(), Release);
                task::Poll::Pending
            },
            SubsRecv::Waiting => task::Poll::Pending,
            SubsRecv::NoSender => task::Poll::Ready(Err(NoSenders)),
            SubsRecv::Received(message) => task::Poll::Ready(Ok(message)),
        }
    }
}

impl<'receiver, T> Future for Subscriber<'receiver, T> {
    type Output = Result<T, NoSenders>;

    fn poll(
        mut self: Pin<&mut Self>,
        ctx: &mut task::Context,
    ) -> task::Poll<Self::Output> {
        let poll = match self.state {
            SubsState::TakeOrphan => unsafe { self.take_orphan() },
            SubsState::Subscribe => unsafe { self.subscribe(ctx.waker()) },
            SubsState::Done => {
                panic!("spmc::receiver::Subscribe polled after done")
            },
        };

        if poll.is_ready() {
            self.state = SubsState::Done;
        }

        poll
    }
}
