//! This module defines the receiver of a SPMC channel.

use crate::{
    error::NoSenders,
    spmc::shared::{ReceiverSubs, Shared},
};
use std::sync::Arc;

/// The receiver handle of a SPMC last-shot channel. Since the channel is a
/// "multi consumer" channel (spMC), it is possible to clone this handle, but
/// it requires a mutable reference. The cost of cloning involves incrementing
/// an atomic variable and making a new small allocation.
pub struct Receiver<T> {
    /// Structure shared with receiver.
    shared: Arc<Shared<T>>,
    /// The receiver handle to the subscription data it owns. To subscribe for
    /// a new message, it uses this object.
    subs: ReceiverSubs<T>,
}

impl<T> Receiver<T> {
    /// Creates a new sender from the given shared structure ARC.
    ///
    /// # Safety
    /// Safe if the shared structure can only be shared with the sender, and it
    /// must control its presence flag correctly (while active set to 1).
    pub(crate) unsafe fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared, subs: ReceiverSubs::new() }
    }

    pub fn try_recv(&self) -> Result<Option<T>, NoSenders> {
        unimplemented!()
    }

    pub async fn recv(&self) -> Result<T, NoSenders> {
        unimplemented!()
    }
}
