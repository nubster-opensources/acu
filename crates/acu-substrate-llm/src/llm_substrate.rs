//! The single LLM-backed substrate, generic over the provider.

use acu_core::{Decision, MindState, Percept, Substrate, SubstrateError};

use crate::language_model::LanguageModel;
use crate::prompt::build_prompt;

/// A [`Substrate`] that decides by completing a prompt with a [`LanguageModel`] provider.
///
/// It owns the whole cognition-to-text mapping. The provider `M` is interchangeable, so the same
/// decision logic runs on Mistral today and on another provider tomorrow.
#[derive(Debug)]
pub struct LlmSubstrate<M: LanguageModel> {
    model: M,
}

impl<M: LanguageModel> LlmSubstrate<M> {
    /// Wraps a provider into a substrate.
    pub fn new(model: M) -> LlmSubstrate<M> {
        LlmSubstrate { model }
    }
}

impl<M: LanguageModel + Sync> Substrate for LlmSubstrate<M> {
    async fn decide(
        &self,
        state: &MindState,
        percept: &Percept,
    ) -> Result<Decision, SubstrateError> {
        let prompt = build_prompt(state, percept);
        let reply = self
            .model
            .complete(&prompt)
            .await
            .map_err(|error| SubstrateError::new(error.to_string()))?;
        Ok(Decision {
            response: reply.trim().to_string(),
        })
    }
}
