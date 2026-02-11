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
            _ => "https://api.anthropic.com/v1",
        };

        Self {
            client: Client::new(),
            api_key,
            provider: provider.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        log::info!("Sending chat request to {}", self.provider);

        let rb = if self.provider == "openai" {
            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
        } else {
            self.client
                .post(format!("{}/messages", self.base_url))
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
        };

        let response = rb
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        // 1. Capture status and raw text body
        let status = response.status();
        let raw_body = response.text().await?;

        // 2. Handle API Errors (400, 401, 404, etc.)
        if !status.is_success() {
            anyhow::bail!("API error (Status {}): {}", status, raw_body);
        }

        // 3. Attempt to decode the JSON
        let chat_response: ChatResponse = serde_json::from_str(&raw_body).map_err(|e| {
            // If streaming is on, raw_body will start with "data: "
            // This error message will now show you exactly what caused the crash
            anyhow::anyhow!("JSON Decode Error: {}. \nRaw Body: {}", e, raw_body)
        })?;

        Ok(chat_response)
    }
    pub async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        // TODO: Implement streaming
        todo!("Streaming implementation")
    }
}
