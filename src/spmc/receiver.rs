use crate::{
    error::NoSenders,
    spmc::shared::{ReceiverSubs, Shared},
};
use std::sync::Arc;

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    subs: ReceiverSubs<T>,
}

impl<T> Receiver<T> {
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
