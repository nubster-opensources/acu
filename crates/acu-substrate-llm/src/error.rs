//! Errors raised by a language-model provider.

use thiserror::Error;

/// An error raised while talking to a language-model provider.
#[derive(Debug, Error)]
pub enum LlmError {
    /// The request to the provider could not be sent or completed.
    #[error("language model request failed: {0}")]
    Request(String),

    /// The provider answered, but the answer could not be turned into usable text.
    #[error("language model returned an unusable response: {0}")]
    Response(String),

    /// A required piece of configuration was missing or invalid.
    #[error("missing or invalid configuration: {0}")]
    Configuration(String),
}
