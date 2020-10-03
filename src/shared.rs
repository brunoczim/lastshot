//! This module provides the shared structure between [`Sender`](crate::Sender)s and
//! [`Receiver`](crate::Receiver)(crate::Receiver)s.

use std::{
    collections::VecDeque,
    fmt,
    future::Future,
    pin::Pin,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::*},
    task,
};
use tokio::sync::{Mutex, MutexGuard};

/// Shared structure between [`Sender`](crate::Sender)s and [`Receiver`](crate::Receiver)(crate::Receiver)s.
pub struct Shared<T> {
    /// Current message.
    message: AtomicPtr<T>,
    /// List of subscriptions of [`Receiver`](crate::Receiver)(crate::Receiver)s.
    subscriptions: Mutex<VecDeque<task::Waker>>,
    /// Count of connected [`Sender`](crate::Sender)s.
    senders: AtomicUsize,
    /// Count of connected [`Receiver`](crate::Receiver)(crate::Receiver)s.
    receivers: AtomicUsize,
}

impl<T> Shared<T> {
    /// Initializes this shared structure accounting one [`Sender`](crate::Sender) and one
    /// [`Receiver`](crate::Receiver)(crate::Receiver).
    pub fn one_sender_one_recv() -> Self {
        Self {
            message: AtomicPtr::new(null_mut()),
            subscriptions: Mutex::new(VecDeque::new()),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
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

    /// Returns how many [`Sender`](crate::Sender)s are connected.
    pub fn senders(&self) -> usize {
        self.senders.load(Relaxed)
    }

    /// Returns how many [`Receiver`](crate::Receiver)(crate::Receiver)s are connected.
    pub fn receivers(&self) -> usize {
        self.receivers.load(Relaxed)
    }

    /// Accounts that a new [`Sender`](crate::Sender)s connected.
    pub fn sender_created(&self) {
        self.senders.fetch_add(1, Relaxed);
    }

    /// Accounts that a new [`Receiver`](crate::Receiver)(crate::Receiver)s connected.
    pub fn receiver_created(&self) {
        self.receivers.fetch_add(1, Relaxed);
    }

    /// Accounts that a [`Sender`](crate::Sender)s disconnected.
    pub fn sender_dropped(&self) {
        let prev = self.senders.fetch_sub(1, Relaxed);
        if prev == 1 {
            self.notify_all();
        }
    }

    /// Accounts that a [`Receiver`](crate::Receiver)(crate::Receiver)s disconnected.
    pub fn receiver_dropped(&self) {
        self.receivers.fetch_sub(1, Relaxed);
    }

    /// Notifies one subscribed [`Receiver`](crate::Receiver)(crate::Receiver). If it is detected all [`Sender`](crate::Sender)s
    /// disconnected, it will notify all of them.
    pub fn notify_one(&self) {
        if let Ok(mut queue) = self.subscriptions.try_lock() {
            if let Some(waker) = queue.pop_front() {
                waker.wake();
            }
        }

        if self.senders.load(Relaxed) == 0 {
            self.notify_all();
        }
    }

    /// Notifies all subscribed [`Receiver`](crate::Receiver)(crate::Receiver)s.
    pub fn notify_all(&self) {
        if let Ok(mut queue) = self.subscriptions.try_lock() {
            while let Some(waker) = queue.pop_front() {
                waker.wake();
            }
        }
    }

    /// Subscribes the current [`Future`] to the channel subscription list.
    pub async fn subscribe(&self) {
        let mut lock_future = self.subscriptions.lock();
        // Safe because we don't move `lock_future` while `pinned` is active.
        let mut pinned = unsafe { Pin::new_unchecked(&mut lock_future) };
        let do_subscribe = Subscribe { access_subs: pinned.as_mut() };
        do_subscribe.await;
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
            .field("receivers", &self.receivers)
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
    }
}

/// A [`Future`] that actually subscribes [`Future`]s to the channel
/// subscription list.
#[derive(Debug)]
struct Subscribe<'fut, 'guard, F>
where
    F: Future<Output = MutexGuard<'guard, VecDeque<task::Waker>>>,
{
    access_subs: Pin<&'fut mut F>,
}

impl<'fut, 'guard, F> Future for Subscribe<'fut, 'guard, F>
where
    F: Future<Output = MutexGuard<'guard, VecDeque<task::Waker>>>,
{
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        ctx: &mut task::Context,
    ) -> task::Poll<Self::Output> {
        self.access_subs
            .as_mut()
            .poll(ctx)
            .map(|mut queue| queue.push_back(ctx.waker().clone()))
    }
}
