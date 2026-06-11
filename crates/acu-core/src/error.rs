//! The error a substrate may return when it cannot produce a decision.

use core::fmt;

/// An error returned by a [`crate::Substrate`] when it cannot produce a decision.
///
/// It is intentionally opaque to any provider detail: it carries a human-readable message and
/// nothing more, so the cognitive core never learns about networks, models, or transports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstrateError {
    message: String,
}

impl SubstrateError {
    /// Creates a substrate error carrying a human-readable message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> SubstrateError {
        SubstrateError {
            message: message.into(),
        }
    }

    /// The human-readable message describing the failure.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SubstrateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "substrate failed to decide: {}", self.message)
    }
}

impl std::error::Error for SubstrateError {}
