//! What the agent decides in response to a percept.

/// A decision the agent commits to in response to a percept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// The response the agent chooses to produce.
    pub response: String,
}
