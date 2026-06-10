//! The episodic memory port and an in-process adapter for tests and early development.

use crate::event::CognitiveEvent;

/// The episodic memory port: an append-only log of everything the agent has lived.
pub trait EventStore {
    /// Appends new events to the end of the log, preserving their order.
    fn append(&mut self, events: Vec<CognitiveEvent>);

    /// Loads the full event history, oldest first.
    fn load(&self) -> Vec<CognitiveEvent>;
}

/// An in-process [`EventStore`] backed by a vector, for tests and early development.
#[derive(Debug, Default)]
pub struct InMemoryEventStore {
    events: Vec<CognitiveEvent>,
}

impl InMemoryEventStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> InMemoryEventStore {
        InMemoryEventStore { events: Vec::new() }
    }
}

impl EventStore for InMemoryEventStore {
    fn append(&mut self, events: Vec<CognitiveEvent>) {
        self.events.extend(events);
    }

    fn load(&self) -> Vec<CognitiveEvent> {
        self.events.clone()
    }
}
