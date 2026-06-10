//! The single place that maps cognition to text: building the prompt sent to a provider.

use acu_core::{MindState, Percept};

/// A prompt addressed to a language-model provider, split into a system and a user part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    /// The standing instructions that frame how the agent should answer.
    pub system: String,
    /// The concrete turn the agent must answer, derived from the percept and recent state.
    pub user: String,
}

/// Builds a [`Prompt`] from the agent's current `state` and the fresh `percept`.
#[must_use]
pub fn build_prompt(state: &MindState, percept: &Percept) -> Prompt {
    let system = "You are the cognitive core of an autonomous unit. \
Reply to the latest message concisely and helpfully."
        .to_string();
    let user = match &state.last_decision {
        Some(last) => format!(
            "Earlier you said: {}\nNow they say: {}",
            last.response, percept.utterance
        ),
        None => percept.utterance.clone(),
    };
    Prompt { system, user }
}
