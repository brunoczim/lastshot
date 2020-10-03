//! This module provides the receiver type.

use crate::{error::NoSenders, Shared};
use std::{fmt, sync::Arc};

/// A receiver handle of a MPMC last-shot channel. It receives messages from the
/// channel. It can be freely cloned and if `T: Send`, also shared. The cost of
/// cloning is the cost of incrementing two atomic variables.
pub struct Receiver<T> {
    /// The shared structure between senders and receivers.
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    /// Creates a new last-shot receiver from the given shared structure.
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    /// Attempts to receive a message from the [`channel`][crate::channel]
    /// and takes it, but does not block if there are no messages.
    ///
    /// If there is a message, it is taken and returned wrapped inside
    /// `Ok(Some(message))`, even if all [`Sender`][crate::sender::Sender]s
    /// have disconnected.
    ///
    /// If there are no messages, but there is at least one
    /// [`Sender`][crate::sender::Sender] connected, it returns `Ok(None)`.
    ///
    /// If there no messages and no [`Sender`][crate::sender::Sender]s
    /// connected, it returns `Err(NoSenders)`.
    pub fn try_recv(&self) -> Result<Option<T>, NoSenders> {
        // Relaxed because `Arc` already does the job of synchronizing.
        if self.shared.senders() == 0 {
            Err(NoSenders)
        } else {
            Ok(self.shared.swap_message(None))
        }
    }

    /// Receives a message from the [`channel`][crate::channel] and takes it.
    ///
    /// While there are no messages, it will wait. If the sender has
    /// disconnected, it returns `Err(NoSenders)`.
    ///
    /// If there is a message, it returns `Ok(message)`, even if all
    /// [`Sender`][crate::sender::Sender]s have disconnected.
    pub async fn recv(&self) -> Result<T, NoSenders> {
        loop {
            if let Some(message) = self.try_recv()? {
                break Ok(message);
            }
            self.shared.subscribe().await;
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("lastshot::Receiver")
            .field("shared", &self.shared)
            .finish()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        // Relaxed because `Arc` already does the job of synchronizing.
        self.shared.receiver_created();
        Self { shared: self.shared.clone() }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Relaxed because `Arc` already does the job of synchronizing.
        self.shared.receiver_dropped();
    }
}
