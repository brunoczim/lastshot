//! This module provides the shared structure between
//! [`Sender`](crate::mpsc::Sender)s and a
//! [`Receiver`](crate::mpsc::Receiver).

use std::{
    fmt,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicU8, AtomicUsize, Ordering::*},
    task,
};

/// Shared structure between [`Sender`](crate::mpsc::Sender)s and a
/// [`Receiver`](crate::mpsc::Receiver).
pub struct Shared<T> {
    /// Current message.
    message: AtomicPtr<T>,
    /// Subscriptions of a [`Receiver`](crate::mpsc::Receiver).
    subscription: AtomicPtr<task::Waker>,
    /// Number of [`Sender`](crate::mpsc::Sender)s active.
    senders: AtomicUsize,
    /// Whether [`Receiver`](crate::mpsc::Receiver) is active or not.
    receiver: AtomicU8,
}

impl<T> Shared<T> {
    /// Initializes this shared structure accounting one
    /// [`Sender`](crate::mpsc::Sender) and one
    /// [`Receiver`](crate::mpsc::Receiver).
    pub fn one_sender() -> Self {
        Self {
            message: AtomicPtr::new(null_mut()),
            subscription: AtomicPtr::new(null_mut()),
            senders: AtomicUsize::new(1),
            receiver: AtomicU8::new(1),
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

    /// Registers a [`Sender`](crate::mpsc::Sender) as created.
    pub fn create_sender(&self) {
        self.senders.fetch_add(1, Relaxed);
    }

    /// Return how many [`Sender`](crate::mpsc::Sender)s are active.
    pub fn senders(&self) -> usize {
        self.senders.load(Relaxed)
    }

    /// Returns whether the [`Receiver`](crate::mpsc::Receiver) is active.
    pub fn receiver(&self) -> bool {
        self.receiver.load(Relaxed) == 1
    }

    /// Registers a sender as dropped. Returns how many
    /// [`Sender`](crate::mpsc::Sender)s are active.
    pub fn drop_sender(&self) -> usize {
        self.senders.fetch_sub(1, Relaxed) - 1
    }

    /// Registers the [`Receiver`](crate::mpsc::Receiver) as dropped.
    pub fn drop_receiver(&self) {
        self.receiver.store(0, Relaxed);
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
        fmt.debug_struct("lastshot::Shared")
            .field("message", &self.message)
            .field("senders", &self.senders)
            .field("receiver", &self.receiver)
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
