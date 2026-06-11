//! Language-model substrate for the Autonomous Cognitive Unit.
//!
//! A single [`LlmSubstrate`] holds the whole mapping between cognition and text: it builds a
//! [`Prompt`] from the mind state and the percept, asks a [`LanguageModel`] provider to complete
//! it, and parses the reply back into a decision. Providers such as [`MistralModel`] are thin
//! adapters of the [`LanguageModel`] port, so swapping providers never touches that mapping.

pub mod error;
pub mod language_model;
pub mod llm_substrate;
pub mod mistral;
pub mod prompt;

pub use error::LlmError;
pub use language_model::LanguageModel;
pub use llm_substrate::LlmSubstrate;
pub use mistral::MistralModel;
pub use prompt::{Prompt, build_prompt};
