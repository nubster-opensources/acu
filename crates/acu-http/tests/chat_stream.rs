//! The chat route must drive one cognitive cycle and stream its stages as Server-Sent Events.
//!
//! These tests pilot the router directly with `tower::oneshot`, with no network and no real
//! provider. A `thinking` event must precede the outcome, the outcome must carry the decision (or
//! the failure), and `done` must close the stream. The cycle's facts must reach the store.

use std::sync::{Arc, Mutex};

use acu_core::{
    CognitiveEvent, Decision, EventStore, InMemoryEventStore, MindState, Percept, ReflexSubstrate,
    Substrate, SubstrateError,
};
use acu_http::{AcuState, router};
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn post_chat(utterance: &str) -> Request<Body> {
    let payload = format!("{{\"utterance\":\"{utterance}\"}}");
    Request::builder()
        .method("POST")
        .uri("/chat")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload))
        .unwrap()
}

async fn body_text(response: axum::response::Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn chat_streams_thinking_then_response_then_done() {
    let state = AcuState::new(ReflexSubstrate, InMemoryEventStore::new());
    let response = router(state).oneshot(post_chat("bonjour")).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    assert!(content_type.starts_with("text/event-stream"));

    let text = body_text(response).await;
    let thinking = text.find("event: thinking").expect("thinking event");
    let answer = text.find("event: response").expect("response event");
    let done = text.find("event: done").expect("done event");

    assert!(text.contains("ack: bonjour"));
    assert!(thinking < answer && answer < done);
}

struct FailingSubstrate;

impl Substrate for FailingSubstrate {
    async fn decide(
        &self,
        _state: &MindState,
        _percept: &Percept,
    ) -> Result<Decision, SubstrateError> {
        Err(SubstrateError::new("mistral unreachable"))
    }
}

#[tokio::test]
async fn chat_streams_an_error_event_when_the_substrate_fails() {
    let state = AcuState::new(FailingSubstrate, InMemoryEventStore::new());
    let response = router(state).oneshot(post_chat("bonjour")).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let text = body_text(response).await;

    assert!(text.contains("event: error"));
    assert!(text.contains("mistral unreachable"));
    assert!(text.contains("event: done"));
    assert!(!text.contains("event: response"));
}

#[derive(Clone)]
struct SpyStore {
    events: Arc<Mutex<Vec<CognitiveEvent>>>,
}

impl EventStore for SpyStore {
    fn append(&mut self, events: Vec<CognitiveEvent>) {
        self.events.lock().unwrap().extend(events);
    }

    fn load(&self) -> Vec<CognitiveEvent> {
        self.events.lock().unwrap().clone()
    }
}

#[tokio::test]
async fn the_cycle_records_the_percept_then_the_decision() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let store = SpyStore {
        events: log.clone(),
    };
    let state = AcuState::new(ReflexSubstrate, store);

    let response = router(state).oneshot(post_chat("bonjour")).await.unwrap();
    // The cognitive cycle runs only as the SSE stream is consumed, so we must drain the body
    // before inspecting what reached the journal.
    let _ = body_text(response).await;

    let recorded = log.lock().unwrap();
    assert_eq!(recorded.len(), 2);
    assert!(matches!(recorded[0], CognitiveEvent::PerceptReceived(_)));
    assert!(matches!(recorded[1], CognitiveEvent::DecisionMade(_)));
}

#[tokio::test]
async fn chat_rejects_a_blank_utterance() {
    let state = AcuState::new(ReflexSubstrate, InMemoryEventStore::new());
    let response = router(state).oneshot(post_chat("   ")).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn chat_rejects_an_oversized_body() {
    let state = AcuState::new(ReflexSubstrate, InMemoryEventStore::new());
    let oversized = "a".repeat(100 * 1024);
    let response = router(state).oneshot(post_chat(&oversized)).await.unwrap();

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
