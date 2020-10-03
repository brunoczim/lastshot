//! This module provides the receiver type.

use crate::{error::NoReceivers, Shared};
use std::{fmt, sync::Arc};

/// A sender handle of a MPMC last-shot channel. It sends messages messages from
/// the channel. It can be freely cloned and if `T: Send`, also shared. The cost
/// of cloning is the cost of incrementing two atomic variables. Sending a
/// message always overwrite the previous message.
pub struct Sender<T> {
    /// The shared structure between senders and receivers.
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    /// Creates a new last-shot receiver from the given shared structure.
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    /// Sends a message through the channel. Sending a message always overwrite
    /// the previous message.
    ///
    /// If all [`Receiver`](crate::Receiver)s disconnected, it returns `Err(NoReceivers)`, and
    /// the error will contain:
    /// - The parameter `message` that the caller attempted to send.
    /// - The value stored in the channel that has never been read, if any.
    ///
    /// Otherwise, if there are receivers, it returns the previous message which
    /// was never read wrapped inside `Ok(Some(message))`.
    pub fn send(&self, message: T) -> Result<Option<T>, NoReceivers<T>> {
        // Relaxed because `Arc` already does the job of synchronizing.
        if self.shared.receivers() == 0 {
            let unreceived = self.shared.swap_message(None);
            Err(NoReceivers { attempt: message, unreceived })
        } else {
            let unreceived = self.shared.swap_message(Some(message));
            self.shared.notify_one();
            Ok(unreceived)
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("lastshot::Sender")
            .field("shared", &self.shared)
            .finish()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // Relaxed because `Arc` already does the job of synchronizing.
        self.shared.sender_created();
        Self { shared: self.shared.clone() }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Relaxed because `Arc` already does the job of synchronizing.
        self.shared.sender_dropped();
    }
}
