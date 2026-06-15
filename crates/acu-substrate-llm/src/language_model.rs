//! The provider port: anything that can complete a prompt into text.

use core::future::Future;

use crate::error::LlmError;
use crate::prompt::Prompt;

/// The provider port: anything that can turn a [`Prompt`] into a textual completion.
///
/// This is the seam that makes the substrate multi-provider. Mistral, a hosted model, or a local
/// runtime are all just implementations, while the cognition-to-text mapping stays in one place.
///
/// The returned future is `Send`, so a substrate awaiting a completion stays `Send` itself and the
/// whole cognitive cycle can run on a multi-threaded async runtime. Providers keep writing
/// `async fn`.
pub trait LanguageModel {
    /// Completes `prompt` and returns the provider's raw textual answer.
    fn complete(&self, prompt: &Prompt) -> impl Future<Output = Result<String, LlmError>> + Send;
}
