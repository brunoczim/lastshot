//! This module provides the receiver type.

use super::Shared;
use crate::error::NoReceivers;
use std::{fmt, sync::Arc};

/// A sender handle of a MPSC last-shot channel. It sends messages messages from
/// the channel. It can be freely cloned and if `T: Send`, also shared. The cost
/// of cloning is the cost of incrementing two atomic variables. Sending a
/// message always overwrite the previous message.
pub struct Sender<T> {
    /// The shared structure between senders and receiver.
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    /// Creates a new last-shot receiver from the given shared structure.
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    /// Sends a message through the channel. Sending a message always overwrite
    /// the previous message (if not received).
    ///
    /// If the [`Receiver`](crate::mpsc::Receiver) disconnected, it
    /// returns `Err(NoReceivers)`, and the error will contain:
    /// - The parameter `message` that the caller attempted to send.
    /// - The value stored in the channel that has never been received, if any.
    ///
    /// Otherwise, if the receiver is connected, it returns the previous message
    /// if never received, wrapped inside `Ok(Some(message))`.
    pub fn send(&self, message: T) -> Result<Option<T>, NoReceivers<T>> {
        if self.shared.connected().receiver {
            let unreceived = self.shared.swap_message(Some(message));
            self.notify();
            Ok(unreceived)
        } else {
            let unreceived = self.shared.swap_message(None);
            Err(NoReceivers { attempt: message, unreceived })
        }
    }

    /// Notifies a subscribed [`Receiver`](crate::mpsc::Receiver).
    fn notify(&self) {
        if let Some(waker) = self.shared.swap_subscription(None) {
            waker.wake();
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
        self.shared.create_sender();
        Self { shared: self.shared.clone() }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        if self.shared.drop_sender().receiver {
            self.notify();
        }
    }
}
