//! The cognitive cycle: one turn of perceive, recall, decide, record.

use crate::decision::Decision;
use crate::error::SubstrateError;
use crate::event::CognitiveEvent;
use crate::mind::MindState;
use crate::percept::Percept;
use crate::store::EventStore;
use crate::substrate::Substrate;

/// Runs one cognitive cycle: perceive, recall, decide, then record what happened.
///
/// The agent loads its history, rebuilds its `MindState`, asks the `substrate` for a decision,
/// then appends the percept and the decision to the log as immutable facts. The returned
/// decision is exactly the one recorded. Generic over the substrate and the store, so the core
/// carries no dependency and no dynamic dispatch.
///
/// # Errors
///
/// Returns the [`SubstrateError`] raised by the substrate when it cannot decide. In that case
/// nothing is appended to the log.
pub async fn step<S: Substrate, E: EventStore>(
    store: &mut E,
    substrate: &S,
    percept: Percept,
) -> Result<Decision, SubstrateError> {
    let state = MindState::replay(&store.load());
    let decision = substrate.decide(&state, &percept).await?;
    store.append(vec![
        CognitiveEvent::PerceptReceived(percept),
        CognitiveEvent::DecisionMade(decision.clone()),
    ]);
    Ok(decision)
}
