//! The reasoning port: the single seam between the cognitive core and how it thinks.

use core::future::Future;

use crate::decision::Decision;
use crate::error::SubstrateError;
use crate::mind::MindState;
use crate::percept::Percept;

/// The reasoning port: given the current mind state and a fresh percept, propose a decision.
///
/// This is the seam that lets the cognitive core run on a reflex rule, a language model, or a
/// stateful learning substrate, without changing the core itself. Deciding is asynchronous and
/// fallible because real substrates may call out over the network.
///
/// The returned future is `Send`, so a cycle can be driven from a multi-threaded async runtime
/// where the future is moved across worker threads. Implementations keep writing `async fn`.
pub trait Substrate {
    /// Proposes a decision for `percept` given the agent's current `state`.
    fn decide(
        &self,
        state: &MindState,
        percept: &Percept,
    ) -> impl Future<Output = Result<Decision, SubstrateError>> + Send;
}
