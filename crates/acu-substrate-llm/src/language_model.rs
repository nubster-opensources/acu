//! The provider port: anything that can complete a prompt into text.

use crate::error::LlmError;
use crate::prompt::Prompt;

/// The provider port: anything that can turn a [`Prompt`] into a textual completion.
///
/// This is the seam that makes the substrate multi-provider. Mistral, a hosted model, or a local
/// runtime are all just implementations, while the cognition-to-text mapping stays in one place.
#[allow(async_fn_in_trait)]
pub trait LanguageModel {
    /// Completes `prompt` and returns the provider's raw textual answer.
    async fn complete(&self, prompt: &Prompt) -> Result<String, LlmError>;
}
