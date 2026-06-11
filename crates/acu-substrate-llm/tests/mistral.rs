//! Red test for the Mistral provider, driven against a mocked HTTP server.
//!
//! No API key and no real network: a mock server stands in for the Mistral endpoint, so we can
//! assert that the provider posts to the right path with the right authorization and parses the
//! reply content, all deterministically.

use acu_substrate_llm::{LanguageModel, MistralModel, Prompt};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn mistral_model_posts_the_prompt_and_returns_the_reply_content() {
    let server = MockServer::start().await;

    let body = serde_json::json!({
        "choices": [
            { "message": { "role": "assistant", "content": "bonjour depuis Mistral" } }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    let model = MistralModel::new("test-key", "mistral-small-latest", server.uri());
    let prompt = Prompt {
        system: "sys".to_string(),
        user: "salut".to_string(),
    };

    let reply = model.complete(&prompt).await.unwrap();

    assert_eq!(reply, "bonjour depuis Mistral");
}

#[tokio::test]
async fn mistral_model_reports_a_request_error_on_a_server_failure() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let model = MistralModel::new("test-key", "mistral-small-latest", server.uri());
    let prompt = Prompt {
        system: "sys".to_string(),
        user: "salut".to_string(),
    };

    let result = model.complete(&prompt).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn mistral_model_includes_status_and_body_in_error_on_non_2xx() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_json(serde_json::json!({"message": "Invalid model"})),
        )
        .mount(&server)
        .await;

    let model = MistralModel::new("test-key", "bad-model", server.uri());
    let prompt = Prompt {
        system: "sys".to_string(),
        user: "salut".to_string(),
    };

    let err = model.complete(&prompt).await.unwrap_err();
    let msg = err.to_string();

    assert!(
        msg.contains("422"),
        "error message should contain the HTTP status code; got: {msg}"
    );
    assert!(
        msg.contains("Invalid model"),
        "error message should contain the response body; got: {msg}"
    );
}
