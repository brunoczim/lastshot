//! A multi-producer single-consumer last-shot channel.

mod shared;
mod sender;
mod receiver;

#[cfg(test)]
mod test;

pub use receiver::Receiver;
pub use sender::Sender;
use shared::Shared;
use std::sync::Arc;

/// Creates a new multi-producer single-consumer (MPSC) "last-shot" channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::one_sender());
    (Sender::new(shared.clone()), Receiver::new(shared))
}
