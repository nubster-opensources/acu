//! The agent's state, rebuilt by replaying its history.

use crate::decision::Decision;
use crate::event::CognitiveEvent;

/// The agent's state. Never stored in place: it is a projection rebuilt by replaying events.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MindState {
    /// How many percepts the agent has received over its whole history.
    pub percepts_seen: u64,
    /// The most recent decision the agent committed to, if any.
    pub last_decision: Option<Decision>,
}

impl MindState {
    /// Rebuilds the mind state by folding the full event history, oldest first.
    #[must_use]
    pub fn replay(events: &[CognitiveEvent]) -> MindState {
        let mut state = MindState::default();
        for event in events {
            match event {
                CognitiveEvent::PerceptReceived(_) => state.percepts_seen += 1,
                CognitiveEvent::DecisionMade(decision) => {
                    state.last_decision = Some(decision.clone());
                }
            }
        }
        state
    }
}
