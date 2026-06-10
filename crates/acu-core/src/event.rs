//! The immutable facts that make up the agent's lived history.

use crate::decision::Decision;
use crate::percept::Percept;

/// An immutable fact in the agent's lived history.
///
/// The whole mind is rebuilt by folding a sequence of these, oldest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CognitiveEvent {
    /// The agent received a percept.
    PerceptReceived(Percept),
    /// The agent committed to a decision.
    DecisionMade(Decision),
}
