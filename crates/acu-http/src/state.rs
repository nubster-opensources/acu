//! The shared engine handle injected into the chat route.

use std::sync::Arc;

use tokio::sync::Mutex;

/// A cloneable handle to the cognitive engine, shared across requests.
///
/// It pairs the reasoning substrate with the episodic store. The store sits behind a
/// [`tokio::sync::Mutex`] because a cognitive cycle holds it across the substrate's `await`,
/// which serializes turns: one mind, one journal, one thought at a time. That is the intended
/// behavior for a single-user host; it should be revisited alongside durable, multi-conversation
/// persistence.
pub struct AcuState<S, E> {
    substrate: Arc<S>,
    store: Arc<Mutex<E>>,
}

impl<S, E> AcuState<S, E> {
    /// Wraps a substrate and a store into a shared, cloneable handle.
    #[must_use]
    pub fn new(substrate: S, store: E) -> AcuState<S, E> {
        AcuState {
            substrate: Arc::new(substrate),
            store: Arc::new(Mutex::new(store)),
        }
    }

    pub(crate) fn substrate(&self) -> Arc<S> {
        self.substrate.clone()
    }

    pub(crate) fn store(&self) -> Arc<Mutex<E>> {
        self.store.clone()
    }
}

impl<S, E> Clone for AcuState<S, E> {
    fn clone(&self) -> AcuState<S, E> {
        AcuState {
            substrate: self.substrate.clone(),
            store: self.store.clone(),
        }
    }
}
