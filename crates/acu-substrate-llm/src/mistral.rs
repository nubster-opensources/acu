//! The default provider: a thin adapter over the Mistral chat completions API.

use core::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::LlmError;
use crate::language_model::LanguageModel;
use crate::prompt::Prompt;

/// A [`LanguageModel`] backed by the Mistral chat completions API.
///
/// A direct HTTPS call to Mistral, no third-party SDK. The API key is read
/// from the `MISTRAL_API_KEY` environment variable.
#[derive(Debug)]
pub struct MistralModel {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl MistralModel {
    /// The default model, chosen for cost.
    pub const DEFAULT_MODEL: &str = "mistral-small-latest";
    /// The default API base URL.
    pub const DEFAULT_BASE_URL: &str = "https://api.mistral.ai";
    /// Default HTTP client timeout applied to every request. Prevents a stalled
    /// provider from blocking the caller indefinitely.
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

    /// Wraps an explicit configuration. Mainly used to point the provider at a mock endpoint in
    /// tests; production code should prefer [`MistralModel::from_env`].
    pub fn new(
        api_key: impl Into<String>,
        model: impl Into<String>,
        base_url: impl Into<String>,
    ) -> MistralModel {
        MistralModel {
            client: reqwest::Client::builder()
                .timeout(Self::DEFAULT_TIMEOUT)
                .build()
                .unwrap_or_default(),
            api_key: api_key.into(),
            model: model.into(),
            base_url: base_url.into(),
        }
    }

    /// Builds a provider from the `MISTRAL_API_KEY` environment variable, using a cheap default
    /// model and the public API base URL.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError::Configuration`] when `MISTRAL_API_KEY` is absent or empty.
    pub fn from_env() -> Result<MistralModel, LlmError> {
        let api_key = std::env::var("MISTRAL_API_KEY")
            .map_err(|_| LlmError::Configuration("MISTRAL_API_KEY is not set".to_string()))?;
        if api_key.trim().is_empty() {
            return Err(LlmError::Configuration(
                "MISTRAL_API_KEY is empty".to_string(),
            ));
        }
        Ok(MistralModel::new(
            api_key,
            Self::DEFAULT_MODEL,
            Self::DEFAULT_BASE_URL,
        ))
    }
}

impl LanguageModel for MistralModel {
    async fn complete(&self, prompt: &Prompt) -> Result<String, LlmError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let request = ChatRequest {
            model: &self.model,
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: &prompt.system,
                },
                ChatMessage {
                    role: "user",
                    content: &prompt.user,
                },
            ],
        };
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|error| LlmError::Request(error.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::Request(format!("{status}: {body}")));
        }
        let completion: ChatResponse = response
            .json()
            .await
            .map_err(|error| LlmError::Response(error.to_string()))?;
        completion
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| LlmError::Response("the provider returned no choices".to_string()))
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}
