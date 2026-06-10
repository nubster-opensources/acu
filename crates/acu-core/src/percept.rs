//! What the agent perceives from the outside world.

/// A single thing the agent perceives, addressed to it from outside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Percept {
    /// The raw utterance addressed to the agent.
    pub utterance: String,
}
