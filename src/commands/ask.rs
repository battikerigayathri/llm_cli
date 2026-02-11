use crate::api::{ChatRequest, LlmClient, Message};
use crate::config::ConfigManager;
use crate::output::OutputFormatter;
use anyhow::{anyhow, bail, Result};

/// Executes the 'ask' command to get a one-shot response from the LLM.
pub async fn execute(
    query: Option<String>,
    file: Option<String>,
    _output: Option<String>,
    model: Option<String>,
    _template: Option<String>,
) -> Result<()> {
    // 1. Initialize Configuration
    // We load the config manager to know which provider and API key name to use
    let config_mgr = ConfigManager::new()?;
    let config = config_mgr.get(); // WORKS: uses your public getter
                                   // 2. Resolve the API Key
                                   // It looks for the variable name defined in your config (e.g., "OPENAI_API_KEY")
    let api_key = config_mgr.get_api_key()?;
    // 3. Resolve the Query Text
    // Priority: Command line argument > File content > Error
    let query_text = if let Some(q) = query {
        q
    } else if let Some(f) = file {
        std::fs::read_to_string(&f).map_err(|e| anyhow!("Failed to read file {}: {}", f, e))?
    } else {
        bail!("Either a query string or a --file path must be provided.");
    };

    // 4. Initialize Client and Formatter
    let client = LlmClient::new(api_key, &config.api.provider);
    let formatter = OutputFormatter::new(
        config.output.syntax_highlighting,
        config.output.markdown_rendering,
    );

    // 5. Build the Request
    // We merge CLI flags with default values from your config.toml
    let request = ChatRequest {
        model: model.unwrap_or_else(|| config.models.default.clone()),
        messages: vec![Message {
            role: "user".to_string(),
            content: query_text,
        }],
        temperature: Some(config.chat.temperature),
        max_tokens: config.chat.max_tokens,       // FIX: Removed Some()
        stream: Some(false),      // FIX: Added Some()
    };

    formatter.print_info(&format!("🚀 Using provider: {}", config.api.provider));

    // 6. Perform the API Call
    match client.chat(request).await {
        Ok(response) => {
            // Use the unified helper method we created in models.rs
            let response_text = response.get_text();
            
            if response_text.is_empty() || response_text == "Error: No response content found" {
                formatter.print_error("Received an empty response from the provider.");
            } else {
                formatter.print_response(&response_text);
            }
        }
        Err(e) => {
            formatter.print_error(&format!("API Call Failed: {}", e));
        }
    }

    Ok(())
}
