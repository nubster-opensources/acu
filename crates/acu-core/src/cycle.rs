//! The cognitive cycle: one turn of perceive, recall, decide, record.

use crate::decision::Decision;
use crate::event::CognitiveEvent;
use crate::mind::MindState;
use crate::percept::Percept;
use crate::store::EventStore;
use crate::substrate::Substrate;

/// Runs one cognitive cycle: perceive, recall, decide, then record what happened.
///
/// The agent loads its history, rebuilds its `MindState`, asks the `substrate` for a decision,
/// then appends the percept and the decision to the log as immutable facts. The returned
/// decision is exactly the one recorded.
pub fn step(store: &mut dyn EventStore, substrate: &dyn Substrate, percept: Percept) -> Decision {
    let state = MindState::replay(&store.load());
    let decision = substrate.decide(&state, &percept);
    store.append(vec![
        CognitiveEvent::PerceptReceived(percept),
        CognitiveEvent::DecisionMade(decision.clone()),
    ]);
    decision
}
