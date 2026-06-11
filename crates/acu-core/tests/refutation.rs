//! Refutation tests for the cognitive core: the heart of brick 1.
//!
//! The architecture stands or falls on one property: the mind state is a faithful,
//! deterministic replay of the lived event log, independent of which substrate produced it.
//! If replaying the journal diverges from what was lived, or if swapping the substrate forces
//! a change to the core, the brick falls.

use acu_core::{
    CognitiveEvent, Decision, EventStore, InMemoryEventStore, MindState, Percept, ReflexSubstrate,
    Substrate, SubstrateError, step,
};

fn percept(utterance: &str) -> Percept {
    Percept {
        utterance: utterance.to_string(),
    }
}

fn decision(response: &str) -> Decision {
    Decision {
        response: response.to_string(),
    }
}

#[test]
fn replay_of_empty_history_is_the_default_mind() {
    let state = MindState::replay(&[]);

    assert_eq!(state, MindState::default());
}

#[test]
fn replay_counts_every_percept_and_keeps_the_last_decision() {
    let events = vec![
        CognitiveEvent::PerceptReceived(percept("bonjour")),
        CognitiveEvent::DecisionMade(decision("salut")),
        CognitiveEvent::PerceptReceived(percept("ca va ?")),
        CognitiveEvent::DecisionMade(decision("oui")),
    ];

    let state = MindState::replay(&events);

    assert_eq!(state.percepts_seen, 2);
    assert_eq!(state.last_decision, Some(decision("oui")));
}

#[tokio::test]
async fn step_records_the_percept_then_the_decision_it_returns() {
    let mut store = InMemoryEventStore::new();
    let substrate = ReflexSubstrate;

    let made = step(&mut store, &substrate, percept("bonjour"))
        .await
        .unwrap();

    let history = store.load();
    assert_eq!(history.len(), 2);
    assert_eq!(
        history[0],
        CognitiveEvent::PerceptReceived(percept("bonjour"))
    );
    assert_eq!(history[1], CognitiveEvent::DecisionMade(made));
}

#[tokio::test]
async fn replayed_state_matches_what_was_lived_with_the_reflex_substrate() {
    let mut store = InMemoryEventStore::new();
    let substrate = ReflexSubstrate;

    step(&mut store, &substrate, percept("un")).await.unwrap();
    step(&mut store, &substrate, percept("deux")).await.unwrap();
    let last = step(&mut store, &substrate, percept("trois"))
        .await
        .unwrap();

    let state = MindState::replay(&store.load());

    assert_eq!(state.percepts_seen, 3);
    assert_eq!(state.last_decision, Some(last));
}

/// A second substrate, defined entirely here, proving the core is substrate-agnostic: it drives
/// the same cycle and the same replay invariant without `acu-core` knowing anything about it.
struct EchoSubstrate;

impl Substrate for EchoSubstrate {
    async fn decide(
        &self,
        _state: &MindState,
        percept: &Percept,
    ) -> Result<Decision, SubstrateError> {
        Ok(Decision {
            response: percept.utterance.clone(),
        })
    }
}

#[tokio::test]
async fn the_substrate_is_swappable_and_replay_still_holds() {
    let mut store = InMemoryEventStore::new();
    let substrate = EchoSubstrate;

    let last = step(&mut store, &substrate, percept("miroir"))
        .await
        .unwrap();

    let state = MindState::replay(&store.load());

    assert_eq!(state.percepts_seen, 1);
    assert_eq!(state.last_decision, Some(last.clone()));
    assert_eq!(last, decision("miroir"));
}
