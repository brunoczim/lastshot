use crate::{
    error::NoReceivers,
    spmc::shared::{Node, Shared},
};
use std::{ptr::NonNull, sync::Arc};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
    front: NonNull<Node<T>>,
}

impl<T> Sender<T> {
    pub(crate) unsafe fn new(
        shared: Arc<Shared<T>>,
        single_node: NonNull<Node<T>>,
    ) -> Self {
        Self { shared, front: single_node }
    }

    pub fn send(&self, message: T) -> Result<(), NoReceivers<T>> {
        unimplemented!()
    }
}
