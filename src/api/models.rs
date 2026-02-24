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
    pub max_completion_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    // Uses alias to handle both OpenAI's "id" and Gemini's "responseId"
    #[serde(alias = "responseId")]
    pub id: String,
    
    // Anthropic format
    pub content: Option<Vec<ContentBlock>>,
    
    // OpenAI format
    pub choices: Option<Vec<Choice>>,
    
    // Google Gemini format
    pub candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
}

#[derive(Debug, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiPart {
    // Made optional because Gemini 3 might send a part with only a thoughtSignature
    pub text: Option<String>,
    #[serde(rename = "thoughtSignature")]
    pub thought_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

impl ChatResponse {
    pub fn get_text(&self) -> String {
        // 1. Try Anthropic path
        if let Some(content) = &self.content {
            return content.iter()
                .filter_map(|block| block.text.as_ref())
                .cloned()
                .collect::<Vec<_>>()
                .join("");
        }

        // 2. Try OpenAI path
        if let Some(choices) = &self.choices {
            if let Some(choice) = choices.first() {
                return choice.message.content.clone();
            }
        }

        // 3. Try Google Gemini path (Gemini 3 Compatible)
        if let Some(candidates) = &self.candidates {
            if let Some(candidate) = candidates.first() {
                // We join ALL parts because Gemini 3 often splits thoughts and text
                let text = candidate.content.parts.iter()
                    .filter_map(|part| part.text.as_ref())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("");
                
                if !text.is_empty() {
                    return text;
                }
            }
        }

        "Error: No response content found".to_string()
    }
}