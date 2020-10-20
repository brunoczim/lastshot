//! A single-producer multi-consumer last-shot channel.

mod shared;
mod sender;
mod receiver;

pub use receiver::Receiver;
pub use sender::Sender;
use shared::Shared;
use std::sync::Arc;

/// Creates a new single-producer multi-consumer (SPMC) "last-shot" channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (shared_inner, single_node) = Shared::one_receiver();
    let shared = Arc::new(shared_inner);
    let sender = unsafe { Sender::new(shared.clone(), single_node) };
    let receiver = unsafe { Receiver::new(shared) };
    (sender, receiver)
}
