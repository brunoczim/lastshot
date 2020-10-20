//! This crate provides an asynchronous "last-shot" channels. A "last-shot"
//! channel is a channel in which the receivers only receive the last message.
//!
//! This is similar to Tokio's [`watch`](tokio::sync::watch). However, Tokio's
//! `watch` always contain a message, it is not possible to take it. You can
//! still listen for updates, but no matter how many times you read, you won't
//! take the value away. In fact, `tokio`'s `watch` clones the message if you
//! need something owned.
//!
//! `lastshot` on the other hand, always take a value away from the channel. And
//! this is always the last sent value. Sending a new value overrides the
//! previous message.
//!
//! This crate provides a Multi-Producer Single-Consumer channel.
//!
//! Perhaps this is not a good name. If you have any ideas, please, suggest it
//! to me.

pub mod error;
pub mod mpsc;
pub mod spmc;
