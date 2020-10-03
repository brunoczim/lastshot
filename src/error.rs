//! This module provides errors used by the channel.

use std::{error::Error, fmt};

/// Returned when there are no
/// [`Receiver`][crate::receiver::Receiver]s
/// connected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoReceivers<T> {
    /// The message that has been atttempted to be sent.
    pub attempt: T,
    /// The message stored in the channel that has never been read, and never
    /// will, since there are no
    /// [`Receiver`][crate::receiver::Receiver]s.
    pub unreceived: Option<T>,
}

impl<T> fmt::Display for NoReceivers<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad("No receiver")
    }
}

impl<T> Error for NoReceivers<T> where T: fmt::Debug {}

/// Returned when there are no [`Sender`][crate::sender::Sender]s connected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoSenders;

impl fmt::Display for NoSenders {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad("No senders")
    }
}

impl Error for NoSenders {}
