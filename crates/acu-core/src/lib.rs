//! Event-sourced cognitive core for the Autonomous Cognitive Unit.
//!
//! The mind is a projection over an append-only log of [`CognitiveEvent`]s. Reasoning sits
//! behind the [`Substrate`] port and storage behind the [`EventStore`] port, so the core
//! depends on nothing technical.

pub mod cycle;
pub mod decision;
pub mod error;
pub mod event;
pub mod mind;
pub mod percept;
pub mod reflex;
pub mod store;
pub mod substrate;

pub use cycle::step;
pub use decision::Decision;
pub use error::SubstrateError;
pub use event::CognitiveEvent;
pub use mind::MindState;
pub use percept::Percept;
pub use reflex::ReflexSubstrate;
pub use store::{EventStore, InMemoryEventStore};
pub use substrate::Substrate;
