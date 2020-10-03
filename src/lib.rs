//! This crate provides an asynchronous "last-shot" channel. A "last-shot"
//! channel is a channel in which the receivers only receive the last message.
//!
//! This is similar to Tokio's [`watch`](tokio::sync::watch). However, Tokio's
//! `watch` always contain a message, it is not possible to take it. You can
//! still listen for updates, but no matter how many times you read, you won't
//! take the value away. In fact, `tokio`s `watch` clones the message if you
//! need it borrowed.
//!
//! `lastshot` on the other hand, always take a value away from the channel. And
//! this is always the last sent value. Sending a new value overrides the
//! previous message.
//!
//! This channel is a multiple producers, multiple consumers channel. Copying
//! [`Sender`](crate::Sender)s and
//! [`Receiver`](crate::Receiver)s is the cost of incrementing
//! two atomic integers.
//!
//! With multiple [`Receiver`]s, however, some [`Receiver`]s might get the
//! message in a different order from which they asked for a message.
//!
//! Perhaps this is not a good name. If you have any ideas, please, suggest it
//! to me.

mod error;
mod shared;
mod sender;
mod receiver;

#[cfg(test)]
mod test;

pub use error::{NoReceivers, NoSenders};
pub use receiver::Receiver;
pub use sender::Sender;
use shared::Shared;
use std::sync::Arc;

/// Creates a new multiple producers multiple consumers (MPMC) "last-shot"
/// channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::one_sender_one_recv());
    (Sender::new(shared.clone()), Receiver::new(shared))
}
