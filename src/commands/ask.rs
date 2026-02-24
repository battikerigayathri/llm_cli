use crate::api::{ChatRequest, LlmClient, Message};
use crate::config::ConfigManager;
use crate::output::OutputFormatter;
use anyhow::Context;
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
    let config_mgr = ConfigManager::new()?;
    let config = config_mgr.get();

    // 2. Determine the model to use
    let model_name = model.unwrap_or_else(|| config.models.default.clone());
    let model_info = config_mgr
        .get_model_info(&model_name)
        .context(format!("Model '{}' not found in config.toml", model_name))?;
    // 3. Get API key for the provider
    let api_key = config_mgr.get_api_key(&model_info.provider)?;

    // 4. Resolve the Query Text
    let query_text = if let Some(q) = query {
        q
    } else if let Some(f) = file {
        std::fs::read_to_string(&f).map_err(|e| anyhow!("Failed to read file {}: {}", f, e))?
    } else {
        bail!("Either a query string or a --file path must be provided.");
    };

    // 5. Initialize Client and Formatter
    let client = LlmClient::new(api_key, &model_info.provider);
    let formatter = OutputFormatter::new(
        config.output.syntax_highlighting,
        config.output.markdown_rendering,
    );

    // 6. Build the Request
    let request = ChatRequest {
        model: model_name.clone(),
        messages: vec![Message {
            role: "user".to_string(),
            content: query_text,
        }],
        temperature: Some(config.chat.temperature),
        max_completion_tokens: config.chat.max_tokens,
        stream: Some(false),
    };

    formatter.print_info(&format!(
        "🚀 Using provider: {} with model: {}",
        model_info.provider, model_name
    ));

    // 6. Perform the API Call
    match client.chat(request).await {
        Ok(response) => {
            let text = response.get_text(); // This uses your new logic from model.rs
            if text.is_empty() {
                println!("(Received empty response from model)");
            } else {
                println!("{}", text); // <-- THIS is what's missing
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
