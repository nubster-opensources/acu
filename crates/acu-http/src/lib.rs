//! HTTP transport for the Autonomous Cognitive Unit.
//!
//! This crate exposes a reusable Axum [`Router`] that drives one cognitive cycle per request and
//! streams its stages as Server-Sent Events. It depends only on `acu-core`: it knows nothing about
//! which substrate reasons, where events are stored, or how the host authenticates callers. The
//! host wires those in via [`AcuState`] and wraps the router with its own authentication and
//! transport security.

mod chat;
mod state;

pub use chat::ChatRequest;
pub use state::AcuState;

use acu_core::{EventStore, Substrate};
use axum::Router;
use axum::routing::post;

/// Builds the cognitive chat router.
///
/// The returned router exposes `POST /chat` and carries `state` internally, so a host can nest it
/// under any path and wrap it with authentication. Authorization and transport security are the
/// host's responsibility, not this crate's.
pub fn router<S, E>(state: AcuState<S, E>) -> Router
where
    S: Substrate + Send + Sync + 'static,
    E: EventStore + Send + 'static,
{
    Router::new()
        .route("/chat", post(chat::chat::<S, E>))
        .with_state(state)
}
