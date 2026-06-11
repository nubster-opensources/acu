//! Red tests for the LLM substrate.
//!
//! The `LanguageModel` seam makes the substrate testable without a network: a stub provider
//! returns a canned reply, and we assert that the substrate maps cognition to text and back, and
//! that it drives the same event-sourced cycle and replay invariant as any other substrate.

use acu_core::{EventStore, InMemoryEventStore, MindState, Percept, Substrate, step};
use acu_substrate_llm::{LanguageModel, LlmError, LlmSubstrate, Prompt, build_prompt};

struct StubModel {
    reply: String,
}

impl LanguageModel for StubModel {
    async fn complete(&self, _prompt: &Prompt) -> Result<String, LlmError> {
        Ok(self.reply.clone())
    }
}

fn percept(utterance: &str) -> Percept {
    Percept {
        utterance: utterance.to_string(),
    }
}

#[test]
fn build_prompt_carries_the_percept_into_the_user_turn() {
    let prompt = build_prompt(&MindState::default(), &percept("quelle heure est-il ?"));

    assert!(prompt.user.contains("quelle heure est-il ?"));
}

#[tokio::test]
async fn llm_substrate_turns_the_trimmed_model_reply_into_a_decision() {
    let substrate = LlmSubstrate::new(StubModel {
        reply: "  bonjour Pierrick  ".to_string(),
    });

    let decision = substrate
        .decide(&MindState::default(), &percept("salut"))
        .await
        .unwrap();

    assert_eq!(decision.response, "bonjour Pierrick");
}

#[tokio::test]
async fn llm_substrate_drives_the_full_cognitive_cycle() {
    let mut store = InMemoryEventStore::new();
    let substrate = LlmSubstrate::new(StubModel {
        reply: "oui".to_string(),
    });

    let decision = step(&mut store, &substrate, percept("ca va ?"))
        .await
        .unwrap();

    assert_eq!(decision.response, "oui");
    let state = MindState::replay(&store.load());
    assert_eq!(state.percepts_seen, 1);
    assert_eq!(state.last_decision, Some(decision));
}
