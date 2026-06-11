//! Live smoke test of the cognitive cycle against the real Mistral API.
//!
//! Ignored by default: it needs a real `MISTRAL_API_KEY` and network access, so it never runs
//! in continuous integration. Run it manually with the key in the environment:
//!
//! ```text
//! cargo test -p acu-substrate-llm --test live_mistral -- --ignored --nocapture
//! ```
//!
//! What this proves is narrow and honest: the event-sourced cycle drives a real language-model
//! provider end to end, and the replay invariant still holds against it. The reply printed here
//! is Mistral speaking, not an identity of its own. A distinct voice would come later from a
//! framing system prompt, recalled memory, and the embodying facade, none of which exist yet.

use acu_core::{EventStore, InMemoryEventStore, MindState, Percept, step};
use acu_substrate_llm::{LlmSubstrate, MistralModel};

#[tokio::test]
#[ignore = "requires a real MISTRAL_API_KEY and network access"]
async fn the_cognitive_cycle_completes_against_the_real_mistral_api() {
    let model = MistralModel::from_env().expect("MISTRAL_API_KEY must be set for the live test");
    let substrate = LlmSubstrate::new(model);
    let mut store = InMemoryEventStore::new();

    let percept = Percept {
        utterance: "Reponds en une phrase pour confirmer que le cycle fonctionne.".to_string(),
    };
    let decision = step(&mut store, &substrate, percept)
        .await
        .expect("the live cognitive cycle should produce a decision");

    assert!(!decision.response.trim().is_empty());
    println!("Mistral, through the ACU cycle: {}", decision.response);

    let state = MindState::replay(&store.load());
    assert_eq!(state.percepts_seen, 1);
    assert_eq!(state.last_decision.as_ref(), Some(&decision));
}
