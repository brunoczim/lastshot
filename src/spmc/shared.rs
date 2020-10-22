//! This module provides shared structures between
//! [`Sender`](crate::spmc::Sender) and [`Receiver`](crate::spmc::Receiver).

use std::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    mem,
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicUsize, Ordering::*},
    task,
};

/// The message presence bit in the [`SubsData`].
///
/// 1000 0000
const MESSAGE_BIT: u8 = 1 << 7;

/// The waker presence bit in the [`SubsData`].
///
/// 0100 0000
const WAKER_BIT: u8 = 1 << 6;

/// A pointer stored in a node to refer to its data.
pub enum NodeDataPtr<T> {
    Orphan(*mut OrphanData<T>),
    /// This is a pointer to a subscription node's data. Must be aligned.
    Subs(*mut SubsData<T>),
}

impl<T> Clone for NodeDataPtr<T> {
    fn clone(&self) -> Self {
        match self {
            &NodeDataPtr::Orphan(ptr) => NodeDataPtr::Orphan(ptr),
            &NodeDataPtr::Subs(ptr) => NodeDataPtr::Subs(ptr),
        }
    }
}

impl<T> Copy for NodeDataPtr<T> {}

impl<T> fmt::Debug for NodeDataPtr<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeDataPtr::Orphan(ptr) => {
                fmt.debug_tuple("Orphan").field(ptr).finish()
            },
            NodeDataPtr::Subs(ptr) => {
                fmt.debug_tuple("Subs").field(ptr).finish()
            },
        }
    }
}

impl<T> NodeDataPtr<T> {
    /// Convenience method for creating a null pointer.
    ///
    /// # Panic
    /// Panics if the actual null pointer is not aligned for an alignment of 2.
    pub fn null() -> Self {
        let null = null_mut();
        assert_eq!(null as usize & 1, 0);
        NodeDataPtr::Orphan(null)
    }

    /// Tests if this node data pointer is null.
    pub fn is_null(self) -> bool {
        match self {
            NodeDataPtr::Orphan(ptr) => ptr.is_null(),
            NodeDataPtr::Subs(ptr) => ptr.is_null(),
        }
    }
}

/// A node data, stored in [`Node`]'s `data` field.
pub struct NodeData<T> {
    /// The pointer to the actual node data.
    pub ptr: NodeDataPtr<T>,
    /// Whether all sides are connected.
    pub connected: bool,
}

impl<T> NodeData<T> {
    /// Encodes this node data in bits.
    pub fn encode(self) -> usize {
        let connected = self.connected as usize;
        let ptr = match self.ptr {
            NodeDataPtr::Orphan(ptr) => ptr as usize,
            NodeDataPtr::Subs(ptr) => ptr as usize | 2,
        };

        ptr | connected
    }

    /// Decodes bits into a node data.
    pub fn decode(bits: usize) -> Self {
        let connected = bits & 1 == 0;
        let ptr = if bits & 2 == 0 {
            NodeDataPtr::Orphan((bits & !3) as *mut _)
        } else {
            NodeDataPtr::Subs((bits & !3) as *mut _)
        };
        Self { ptr, connected }
    }
}

impl<T> Clone for NodeData<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr, connected: self.connected }
    }
}

impl<T> Copy for NodeData<T> {}

impl<T> fmt::Debug for NodeData<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("NodeData")
            .field("ptr", &self.ptr)
            .field("connected", &self.connected)
            .finish()
    }
}

/// Subscription data handle owned by the receiver.
pub struct ReceiverSubs<T> {
    /// The pointer to subscription data.
    data: NonNull<SubsData<T>>,
}

impl<T> fmt::Debug for ReceiverSubs<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ReceiverSubs").field("data", self.data()).finish()
    }
}

impl<T> ReceiverSubs<T> {
    /// Creates a new receiver's subscrition data uninitialized.
    pub fn new() -> Self {
        let alloc = Box::new(SubsData {
            waker: UnsafeCell::new(MaybeUninit::uninit()),
            message: UnsafeCell::new(MaybeUninit::uninit()),
            count: AtomicU8::new(1),
        });
        Self { data: NonNull::from(Box::leak(alloc)) }
    }

    /// Returns a reference to the subscription data.
    pub fn data(&self) -> &SubsData<T> {
        // Safe because we only allocate the pointer through `Box` and never get
        // it from outside.
        unsafe { self.data.as_ref() }
    }

    /// Creates a sender-owned handle to subscription data.
    ///
    /// # Safety
    /// This is safe if no sender handle to subs data is active.
    pub unsafe fn sender_subs(&self) -> SenderSubs<T> {
        self.data().count.fetch_add(1, Acquire);
        SenderSubs { data: self.data }
    }
}

impl<T> Drop for ReceiverSubs<T> {
    fn drop(&mut self) {
        // Safe because we don't use the contents of the data.
        unsafe {
            if self.data().drop_handle() {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}

/// Subscription data handle owned by the sender.
pub struct SenderSubs<T> {
    /// The pointer to subscription data.
    data: NonNull<SubsData<T>>,
}

impl<T> fmt::Debug for SenderSubs<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SenderSubs").field("data", self.data()).finish()
    }
}

impl<T> SenderSubs<T> {
    /// Creates a sender subscription data from a raw, non-null pointer to the
    /// data.
    ///
    /// # Safety
    /// Safe if the pointer `data` was obtained by [`SenderSubs::into_raw`], and
    /// the pointer has not been used with this method since then.
    pub unsafe fn from_raw(data: NonNull<SubsData<T>>) -> Self {
        Self { data }
    }

    /// Gets the raw pointer to the data and "forgets" to drop this sender
    /// handle to subscription data.
    pub fn into_raw(self) -> NonNull<SubsData<T>> {
        let data = self.data;
        mem::forget(self);
        data
    }

    /// Returns a reference to the subscription data.
    pub fn data(&self) -> &SubsData<T> {
        // Safe because we only allocate the pointer through `Box` and even when
        // we get it from outside, it must have been obtained from
        // [`SenderSubs::into_raw`].
        unsafe { self.data.as_ref() }
    }

    /// Sends a message to the receiver who owns this subscription.
    ///
    /// # Safety
    /// Safe if the receiver who owns the receiver-side is indeed subscribed.
    pub unsafe fn send(self, message: T) {
        let data = self.data();
        (*data.message.get()).as_mut_ptr().write(message);
        let waker = (*data.waker.get()).as_mut_ptr().read();
        data.count.fetch_xor(MESSAGE_BIT | WAKER_BIT, Release);
        drop(self);
        waker.wake();
    }
}

impl<T> Drop for SenderSubs<T> {
    fn drop(&mut self) {
        // Safe because we don't use the contents of the data.
        unsafe {
            if self.data().drop_handle() {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}

/// An orphan node's data. An orphan node is a node whose message is not owned
/// by a subscribed task (i.e. not received).
#[repr(align(/* at least */ 4))]
#[derive(Debug)]
pub struct OrphanData<T> {
    /// The message contents.
    pub message: T,
}

/// A subscription node's data. A subscription node is a node whose message is
/// owned by a subscribed task (i.e. received).
#[repr(align(/* at least */ 4))]
pub struct SubsData<T> {
    /// The waker. Potentially uninitialized. While subscribed, should be
    /// initialized by the receiver to be consumed by the sender. That is, when
    /// a receiver subscribes and sets `count` waker bit to 1, it should be
    /// initialized, but the sender might uninitialize it before setting the
    /// `count` waker bit to 0.
    pub waker: UnsafeCell<MaybeUninit<task::Waker>>,
    /// The message. Potentially uninitialized. Should be initialized while a
    /// message has been sent and the receiver did not read it. That is, when
    /// a sender sends message and set the `count` message bit to 1, it should
    /// be initialized, but the receiver might uninitialize it before setting
    /// the `count` waker bit to 0.
    pub message: UnsafeCell<MaybeUninit<T>>,
    /// Count of connected sides. The lower bits are always set to either 1, 2
    /// or 0 if dropping. Higher bits indicate whether the waker is initialized
    /// and whether the message is initialized (the "waker bit" and the
    /// "message bit").
    pub count: AtomicU8,
}

impl<T> fmt::Debug for SubsData<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SubsData")
            .field("count", &self.count.load(Relaxed))
            .finish()
    }
}

impl<T> SubsData<T> {
    /// Registers a handle as being dropped.
    ///
    /// # Safety
    /// Safe if each handle only calls this method once and does not access the
    /// potentially uninitialized, even if the higher `count` bits are set.
    unsafe fn drop_handle(&self) -> bool {
        let count = self.count.fetch_sub(1, Release);
        if count & MESSAGE_BIT != 0 {
            let ptr = self.message.get();
            (&mut *ptr).as_mut_ptr().drop_in_place();
        }
        if count & WAKER_BIT != 0 {
            let ptr = self.waker.get();
            (&mut *ptr).as_mut_ptr().drop_in_place();
        }
        count & 1 == 1
    }
}

/// A node in the subscription queue.
pub struct Node<T> {
    /// Pointer to data. The `usize` should be decoded into a `NodeDataPtr`.
    pub data: AtomicUsize,
    /// The next node in the queue's list.
    pub next: AtomicPtr<Node<T>>,
    /// Marker so that we are allowed to have the type parameter `T`.
    pub _marker: PhantomData<*mut T>,
}

impl<T> fmt::Debug for Node<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            .field("data", &self.data.load(Relaxed))
            .field("next", &self.next.load(Relaxed))
            .finish()
    }
}

/// A shared structure between [`Sender`](crate::spmc::Sender) and
/// [`Receiver`](crate::spmc::Receiver).
pub struct Shared<T> {
    /// The back of the subscription queue.
    back: AtomicPtr<Node<T>>,
    /// Whether the sender is connected.
    sender: AtomicBool,
    /// Count of receivers.
    receivers: AtomicUsize,
}

impl<T> fmt::Debug for Shared<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Shared")
            .field("back", &self.back.load(Relaxed))
            .field("sender", &self.sender.load(Relaxed))
            .field("receivers", &self.receivers.load(Relaxed))
            .finish()
    }
}

impl<T> Shared<T> {
    /// Creates a shared structure
    pub fn one_receiver() -> (Self, NonNull<Node<T>>) {
        let null = NodeDataPtr::<T>::null();
        let node_data = NodeData { ptr: null, connected: true };
        let single_node = Box::new(Node {
            data: AtomicUsize::new(node_data.encode()),
            next: AtomicPtr::default(),
            _marker: PhantomData,
        });
        let single_node_ptr = NonNull::from(Box::leak(single_node));
        let this = Self {
            back: AtomicPtr::new(single_node_ptr.as_ptr()),
            sender: AtomicBool::new(true),
            receivers: AtomicUsize::new(1),
        };

        (this, single_node_ptr)
    }

    /// Returns a reference to the back.
    pub fn back(&self) -> &AtomicPtr<Node<T>> {
        &self.back
    }

    /// Returns whether the sender is connected.
    pub fn sender(&self) -> bool {
        self.sender.load(Relaxed)
    }

    /// Returns how many receivers are connected.
    pub fn receivers(&self) -> usize {
        self.receivers.load(Relaxed)
    }

    /// Registers the creation of a new receiver.
    pub fn create_receiver(&self) {
        self.receivers.fetch_add(1, Relaxed);
    }

    /// Registers the sender being dropped.
    ///
    /// # Safety
    /// Safe only if called by the sender, only once, and if there is only one
    /// sender.
    pub unsafe fn drop_sender(&self) {
        self.sender.store(false, Relaxed)
    }

    /// Registers a receiver being dropped.
    ///
    /// # Safety
    /// Safe only if called by the receiver, only once by receiver.
    pub unsafe fn drop_receivers(&self) -> usize {
        self.receivers.fetch_sub(1, Relaxed) - 1
    }
}
