//! This module provides the receiver type.

use super::Shared;
use crate::error::NoSenders;
use std::{fmt, future::Future, pin::Pin, sync::Arc, task};

/// A receiver handle of a MPSC last-shot channel. It receives messages from the
/// channel. It cannot be cloned and needs a mutable reference.
pub struct Receiver<T> {
    /// The shared structure between senders and receiver.
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    /// Creates a new last-shot receiver from the given shared structure.
    pub(crate) fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    /// Attempts to receive a message from the [`channel`](crate::mpsc::channel)
    /// and takes it, but does not block if there are no messages.
    ///
    /// If there is a message, it is taken and returned wrapped inside
    /// `Ok(Some(message))`, even if all [`Sender`](crate::mpsc::Sender)s
    /// have disconnected.
    ///
    /// If there is no message, but there is at least one
    /// [`Sender`](crate::mpsc::Sender) connected, it returns `Ok(None)`.
    ///
    /// If there no messages and no [`Sender`](crate::mpsc::Sender)s
    /// connected, it returns `Err(NoSenders)`.
    pub fn try_recv(&mut self) -> Result<Option<T>, NoSenders> {
        if self.shared.connected().senders > 0 {
            Ok(self.shared.swap_message(None))
        } else {
            Err(NoSenders)
        }
    }

    /// Receives a message from the [`channel`](crate::mpsc::channel) and takes
    /// it.
    ///
    /// While there are no messages, it will wait. If all the senders have
    /// disconnected, it returns `Err(NoSenders)`.
    ///
    /// If there is a message, it returns `Ok(message)`, even if all
    /// [`Sender`](crate::mpsc::Sender)s have disconnected.
    pub async fn recv(&mut self) -> Result<T, NoSenders> {
        let subscribe = Subscribe { subscribed: false, receiver: self };
        subscribe.await
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("lastshot::Receiver")
            .field("shared", &self.shared)
            .finish()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.drop_receiver();
    }
}

/// A [`Future`] that subscribes [`Future`]s to the channel subscription list.
#[derive(Debug)]
struct Subscribe<'receiver, T> {
    subscribed: bool,
    receiver: &'receiver mut Receiver<T>,
}

impl<'receiver, T> Future for Subscribe<'receiver, T> {
    type Output = Result<T, NoSenders>;

    fn poll(
        mut self: Pin<&mut Self>,
        ctx: &mut task::Context,
    ) -> task::Poll<Self::Output> {
        if !self.subscribed {
            self.subscribed = true;
            self.receiver.shared.swap_subscription(Some(ctx.waker().clone()));
        }

        match self.receiver.try_recv() {
            Ok(Some(val)) => task::Poll::Ready(Ok(val)),
            Ok(None) => task::Poll::Pending,
            Err(error) => task::Poll::Ready(Err(error)),
        }
    }
}
