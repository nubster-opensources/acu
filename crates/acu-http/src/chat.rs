//! The chat route: one POST drives one cognitive cycle, streamed as Server-Sent Events.

use std::convert::Infallible;

use acu_core::{Decision, EventStore, Percept, Substrate, SubstrateError, step};
use axum::Json;
use axum::extract::State;
use axum::response::sse::{Event, Sse};
use futures_core::Stream;
use serde::Deserialize;

use crate::state::AcuState;

/// An incoming chat message from the caller.
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// What the user said to the agent.
    pub utterance: String,
}

/// Drives one cognitive cycle for `request` and streams its stages as Server-Sent Events.
///
/// The stream emits `thinking` as soon as the request arrives, then either `response` carrying the
/// decision or `error` carrying the failure message, and finally `done`. The substrate's call is
/// awaited while the store is locked, so turns are serialized: one mind, one journal, one thought
/// at a time.
pub(crate) async fn chat<S, E>(
    State(state): State<AcuState<S, E>>,
    Json(request): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>>
where
    S: Substrate + Send + Sync + 'static,
    E: EventStore + Send + 'static,
{
    let substrate = state.substrate();
    let store = state.store();
    let percept = Percept {
        utterance: request.utterance,
    };

    let events = async_stream::stream! {
        yield Ok::<_, Infallible>(stage("thinking"));

        let outcome = {
            let mut journal = store.lock().await;
            step(&mut *journal, substrate.as_ref(), percept).await
        };

        match outcome {
            Ok(decision) => yield Ok(responded(&decision)),
            Err(error) => yield Ok(failed(&error)),
        }

        yield Ok(stage("done"));
    };

    Sse::new(events)
}

/// A bare lifecycle event carrying only its name (`thinking`, `done`).
fn stage(name: &str) -> Event {
    Event::default().event(name).data("{}")
}

/// The `response` event carrying the agent's decision.
fn responded(decision: &Decision) -> Event {
    let data = serde_json::json!({ "response": decision.response }).to_string();
    Event::default().event("response").data(data)
}

/// The `error` event carrying the substrate's failure message.
fn failed(error: &SubstrateError) -> Event {
    let data = serde_json::json!({ "message": error.message() }).to_string();
    Event::default().event("error").data(data)
}
