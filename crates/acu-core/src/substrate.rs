//! The reasoning port: the single seam between the cognitive core and how it thinks.

use crate::decision::Decision;
use crate::mind::MindState;
use crate::percept::Percept;

/// The reasoning port: given the current mind state and a fresh percept, propose a decision.
///
/// This is the seam that lets the cognitive core run on a reflex rule, a language model, or a
/// stateful learning substrate, without changing the core itself.
pub trait Substrate {
    /// Proposes a decision for `percept` given the agent's current `state`.
    fn decide(&self, state: &MindState, percept: &Percept) -> Decision;
}
