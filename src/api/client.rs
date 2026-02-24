use super::models::{ChatRequest, ChatResponse};
use anyhow::Result;
use futures::stream::Stream;
use reqwest::Client;
use std::pin::Pin;

pub struct LlmClient {
    client: Client,
    api_key: String,
    provider: String,
    base_url: String,
}

impl LlmClient {
    pub fn new(api_key: String, provider: &str) -> Self {
        let base_url = match provider {
            "anthropic" => "https://api.anthropic.com/v1",
            "openai" => "https://api.openai.com/v1",
            "google" => "https://generativelanguage.googleapis.com/v1beta",
            _ => "https://api.openai.com/v1",
        };

        Self {
            client: Client::new(),
            api_key,
            provider: provider.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        log::info!(
            "Sending chat request to {} with model {}",
            self.provider,
            request.model
        );

        let (url, headers, body) = match self.provider.as_str() {
            "openai" => self.build_openai_request(&request),
            "anthropic" => self.build_anthropic_request(&request),
            "google" => self.build_google_request(&request),
            _ => anyhow::bail!("Unsupported provider: {}", self.provider),
        };

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("API error (Status {}): {}", status, raw_body);
        }

        let chat_response: ChatResponse = serde_json::from_str(&raw_body)
            .map_err(|e| anyhow::anyhow!("JSON Decode Error: {}. \nRaw Body: {}", e, raw_body))?;

        Ok(chat_response)
    }

    fn build_openai_request(
        &self,
        request: &ChatRequest,
    ) -> (String, reqwest::header::HeaderMap, serde_json::Value) {
        let url = format!("{}/chat/completions", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_completion_tokens": request.max_completion_tokens,
            "temperature": request.temperature,
            "stream": request.stream
        });

        (url, headers, body)
    }

    fn build_anthropic_request(
        &self,
        request: &ChatRequest,
    ) -> (String, reqwest::header::HeaderMap, serde_json::Value) {
        let url = format!("{}/messages", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse().unwrap());
        headers.insert("anthropic-version", "2023-06-01".parse().unwrap());

        let body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
        "max_completion_tokens": request.max_completion_tokens,
            "temperature": request.temperature,
            "stream": request.stream
        });

        (url, headers, body)
    }

    fn build_google_request(
        &self,
        request: &ChatRequest,
    ) -> (String, reqwest::header::HeaderMap, serde_json::Value) {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, request.model, self.api_key
        );
        let headers = reqwest::header::HeaderMap::new();

        // Convert messages to Gemini format
        let contents = request
            .messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "parts": [{"text": msg.content}],
                    "role": if msg.role == "user" { "user" } else { "model" }
                })
            })
            .collect::<Vec<_>>();

        let body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "temperature": request.temperature,
                "maxOutputTokens": request.max_completion_tokens
            }
        });

        (url, headers, body)
    }
    pub async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        // TODO: Implement streaming
        todo!("Streaming implementation")
    }
}
