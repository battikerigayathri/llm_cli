use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    // Anthropic format
    pub content: Option<Vec<ContentBlock>>, 
    // OpenAI format
    pub choices: Option<Vec<Choice>>,       
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

// New helper structs for OpenAI's nested format
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

impl ChatResponse {
    /// A unified way to get the text regardless of which provider responded
    pub fn get_text(&self) -> String {
        // Try Anthropic path
        if let Some(content) = &self.content {
            if let Some(block) = content.first() {
                if let Some(text) = &block.text {
                    return text.clone();
                }
            }
        }

        // Try OpenAI path
        if let Some(choices) = &self.choices {
            if let Some(choice) = choices.first() {
                return choice.message.content.clone();
            }
        }

        "Error: No response content found".to_string()
    }
}