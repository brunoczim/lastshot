//! This module provides the shared structure between
//! [`Sender`](crate::mpsc::Sender)s and a
//! [`Receiver`](crate::mpsc::Receiver).

use std::{
    fmt,
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::*},
    task,
};

/// Mask used to select a receiver bit in Connected.
const RECEIVER_MASK: usize = !SENDERS_MASK;
///
/// Mask used to select senders bits in Connected.
const SENDERS_MASK: usize = !0 >> 1;

/// Shared structure between [`Sender`](crate::mpsc::Sender)s and a
/// [`Receiver`](crate::mpsc::Receiver).
pub struct Shared<T> {
    /// Current message.
    message: AtomicPtr<T>,
    /// Subscriptions of a [`Receiver`](crate::mpsc::Receiver).
    subscription: AtomicPtr<task::Waker>,
    /// Number of [`Sender`](crate::mpsc::Sender)s active and if the
    /// [`Receiver`](crate::mpsc::Receiver) is active.
    connected: AtomicUsize,
    /// To say we own a `T`.
    _marker: PhantomData<T>,
}

impl<T> Shared<T> {
    /// Initializes this shared structure accounting one
    /// [`Sender`](crate::mpsc::Sender) and one
    /// [`Receiver`](crate::mpsc::Receiver).
    pub fn one_sender() -> Self {
        let connected = Connected { senders: 1, receiver: true };
        Self {
            message: AtomicPtr::new(null_mut()),
            subscription: AtomicPtr::new(null_mut()),
            connected: AtomicUsize::new(connected.encode()),
            _marker: PhantomData,
        }
    }

    /// Swaps a stored message with the given parameter.
    pub fn swap_message(&self, message: Option<T>) -> Option<T> {
        let ordering = if message.is_none() { Acquire } else { AcqRel };
        let new_ptr = message.map(Box::new).map_or(null_mut(), Box::into_raw);
        let old_ptr = self.message.swap(new_ptr, ordering);

        if old_ptr.is_null() {
            None
        } else {
            // Safe because no one ever reads this pointer without removing it
            // from the message `AtomicPtr`.
            //
            // Also, we checked for null.
            //
            // Also, the pointer we put here is from Box, not one byte before,
            // nor one byte after.
            //
            // Also, we choosed the ordering carefully.
            let boxed = unsafe { Box::from_raw(old_ptr) };
            Some(*boxed)
        }
    }

    /// Swaps a subscription with the given parameter.
    pub fn swap_subscription(
        &self,
        subs: Option<task::Waker>,
    ) -> Option<task::Waker> {
        let ordering = if subs.is_none() { Acquire } else { AcqRel };
        let new_ptr = subs.map(Box::new).map_or(null_mut(), Box::into_raw);
        let old_ptr = self.subscription.swap(new_ptr, ordering);

        if old_ptr.is_null() {
            None
        } else {
            // Safe because no one ever reads this pointer without removing it
            // from the message `AtomicPtr`.
            //
            // Also, we checked for null.
            //
            // Also, the pointer we put here is from Box, not one byte before,
            // nor one byte after.
            //
            // Also, we choosed the ordering carefully.
            let boxed = unsafe { Box::from_raw(old_ptr) };
            Some(*boxed)
        }
    }

    /// Returns info about the connection of the
    /// [`Sender`](crate::mpsc::Sender)s and the
    /// [`Receiver`](crate::mpsc::Receiver).
    pub fn connected(&self) -> Connected {
        Connected::decode(self.connected.load(Relaxed))
    }

    /// Registers a [`Sender`](crate::mpsc::Sender) as created.
    pub fn create_sender(&self) -> Connected {
        let mut connected =
            Connected::decode(self.connected.fetch_add(1, Relaxed));
        connected.senders += 1;
        connected
    }

    /// Registers a [`Sender`](crate::mpsc::Sender) as dropped. Returns
    /// connected data after accouting the drop.
    pub fn drop_sender(&self) -> Connected {
        let mut connected =
            Connected::decode(self.connected.fetch_sub(1, Relaxed));
        connected.senders -= 1;
        connected
    }

    /// Registers the [`Receiver`](crate::mpsc::Receiver) as dropped. Returns
    /// connected data after accouting the drop.

    pub fn drop_receiver(&self) -> Connected {
        let bits = self.connected.fetch_and(!RECEIVER_MASK, Relaxed);
        let mut connected = Connected::decode(bits);
        connected.receiver = false;
        connected
    }
}

// Safe because `T` is never shared, we never actually read `T` while it is
// stored in the channel.
//
// We never do any racy operation.
unsafe impl<T> Send for Shared<T> where T: Send {}
unsafe impl<T> Sync for Shared<T> where T: Send {}

impl<T> fmt::Debug for Shared<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let connected = Connected::decode(self.connected.load(Relaxed));
        fmt.debug_struct("lastshot::Shared")
            .field("message", &self.message)
            .field("connected", &connected)
            .finish()
    }
}

impl<T> Drop for Shared<T> {
    fn drop(&mut self) {
        let ptr = *self.message.get_mut();
        if !ptr.is_null() {
            // Safe because every piece of code we wrote won't read the pointer
            // while it still is there. Everybody removes it before reading its
            // contents.
            //
            // Also, we checked for null.
            unsafe {
                Box::from_raw(ptr);
            }
        }

        let ptr = *self.subscription.get_mut();
        if !ptr.is_null() {
            // Safe because every piece of code we wrote won't read the pointer
            // while it still is there. Everybody removes it before reading its
            // contents.
            //
            // Also, we checked for null.
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}

/// Info about the connection of the [`Sender`](crate::mpsc::Sender)s and the
/// [`Receiver`](crate::mpsc::Receiver).
#[derive(Debug, Clone, Copy)]
pub struct Connected {
    /// How many senders are active.
    pub senders: usize,
    /// Whether the receiver is active.
    pub receiver: bool,
}

impl Connected {
    /// Decodes connection info from bits.
    fn decode(bits: usize) -> Self {
        Self {
            receiver: bits & RECEIVER_MASK != 0,
            senders: bits & SENDERS_MASK,
        }
    }

    /// Encodes connection info into bits.
    fn encode(self) -> usize {
        let receiver = if self.receiver { RECEIVER_MASK } else { 0 };
        receiver | self.senders
    }
}
