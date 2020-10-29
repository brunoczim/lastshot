//! This module provides shared structures between
//! [`Sender`](crate::spmc::Sender) and [`Receiver`](crate::spmc::Receiver).

use crate::spmc::node::{Node, NodeData, NodeDataPtr};
use std::{
    fmt,
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::*},
};

/// Mask used to select a receiver bit in Connected.
const SENDER_MASK: usize = !RECEIVERS_MASK;

/// Mask used to select senders bits in Connected.
const RECEIVERS_MASK: usize = !0 >> 1;

/// A shared structure between [`Sender`](crate::spmc::Sender) and
/// [`Receiver`](crate::spmc::Receiver).
pub struct Shared<T> {
    /// The back of the subscription queue.
    back: AtomicPtr<Node<T>>,
    /// Number of [`Receiver`](crate::mpsc::Receiver)s active and if the
    /// [`Sender`](crate::mpsc::Sender) is active.
    connected: AtomicUsize,
}

impl<T> fmt::Debug for Shared<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let connected = Connected::decode(self.connected.load(Relaxed));
        fmt.debug_struct("Shared")
            .field("back", &self.back.load(Relaxed))
            .field("connected", &connected)
            .finish()
    }
}

impl<T> Shared<T> {
    /// Creates a shared structure accounting for one receiver (and a connected
    /// sender). Returns also the pointer to the single created node.
    pub fn one_receiver() -> (Self, NonNull<Node<T>>) {
        let null = NodeDataPtr::<T>::null();
        let node_data = NodeData { ptr: null, connected: true };
        let single_node = Box::new(Node {
            data: AtomicUsize::new(node_data.encode()),
            next: AtomicPtr::default(),
            _marker: PhantomData,
        });
        let single_node_ptr = NonNull::from(Box::leak(single_node));
        let connected = Connected { receivers: Some(1), sender: true };
        let this = Self {
            back: AtomicPtr::new(single_node_ptr.as_ptr()),
            connected: AtomicUsize::from(connected.encode()),
        };

        (this, single_node_ptr)
    }

    /// Returns a reference to the back.
    pub fn back(&self) -> &AtomicPtr<Node<T>> {
        &self.back
    }

    #[allow(dead_code)]
    /// Returns info about the connection of the
    /// [`Sender`](crate::spmc::Sender) and the
    /// [`Receiver`](crate::spmc::Receiver)s.
    pub fn connected(&self) -> Connected {
        Connected::decode(self.connected.load(Relaxed))
    }

    /// Registers a [`Sender`](crate::spmc::Sender) as created.
    pub fn create_receiver(&self) -> Connected {
        let mut bits = self.connected.fetch_add(1, Relaxed);
        bits -= 1;
        Connected::decode(bits)
    }

    /// Registers a [`Receiver`](crate::spmc::Receiver) as dropped. Returns
    /// connected data after accouting the drop.
    ///
    /// # Safety
    /// Safe only if called by the sender, only once, and if there is only one
    /// sender.
    pub unsafe fn drop_receiver(&self) -> Connected {
        let mut bits = self.connected.fetch_sub(1, Relaxed);
        bits -= 1;
        Connected::decode(bits)
    }

    pub unsafe fn drop_receivers(&self) -> Connected {
        self.drop_receiver()
    }

    /// Registers the [`Sender`](crate::spmc::Sender) as dropped. Returns
    /// connected data after accouting the drop.
    ///
    /// # Safety
    /// Safe only if called by the receiver, only once by receiver.
    pub unsafe fn drop_sender(&self) -> Connected {
        let bits = self.connected.fetch_and(!SENDER_MASK, Relaxed);
        let mut connected = Connected::decode(bits);
        connected.sender = false;
        connected
    }
}

/// Info about the connection of the [`Sender`](crate::spmc::Sender) and the
/// [`Receiver`](crate::spmc::Receiver)s.
#[derive(Debug, Clone, Copy)]
pub struct Connected {
    /// How many receivers are active.
    pub receivers: Option<usize>,
    /// Whether the sender is active.
    pub sender: bool,
}

impl Connected {
    /// Decodes connection info from bits.
    fn decode(bits: usize) -> Self {
        Self {
            sender: bits & SENDER_MASK != 0,
            receivers: (bits & RECEIVERS_MASK).checked_sub(1),
        }
    }

    /// Encodes connection info into bits.
    fn encode(self) -> usize {
        let sender = if self.sender { SENDER_MASK } else { 0 };
        let receivers = match self.receivers {
            Some(count) => count + 1,
            None => 0,
        };
        sender | receivers
    }
}
