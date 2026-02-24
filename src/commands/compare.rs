use crate::api::client::LlmClient;
use crate::api::models::{ChatRequest, Message};
use crate::config::manager::ConfigManager;
use anyhow::Context;
use colored::*;
use futures::future::join_all;
use std::time::Instant;
pub async fn execute(query: String, models: Vec<String>) -> anyhow::Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut tasks = Vec::new();

    println!("{}", "🚀 Comparing models...".bold().cyan());

    for model_name in models {
        let query_clone = query.clone();
        let model_info = config_manager
            .get_model_info(&model_name)
            .context(format!("Model '{}' not found in config", model_name))?
            .clone();
        let api_key = config_manager.get_api_key(&model_info.provider)?;
        let provider = model_info.provider.clone();

        tasks.push(tokio::spawn(async move {
            let client = LlmClient::new(api_key, &provider);

            let request = ChatRequest {
                model: model_name.clone(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: query_clone,
                }],
                // FIX 1: Removed 'Some()' because your struct expects raw types
                temperature: Some(0.7),
                max_completion_tokens: 4096,
                stream: Some(false),
            };

            let start = Instant::now();
            let response = client.chat(request).await;
            let duration = start.elapsed();

            // Use the unified get_text method that handles all providers
            let response_text = match response {
                Ok(res) => res.get_text(),
                Err(e) => format!("Error: {}", e),
            };

            Ok::<(String, String, std::time::Duration), anyhow::Error>((model_name, response_text, duration))
        }));
    }

    let results = join_all(tasks).await;

    for task_result in results {
        if let Ok(Ok((name, response_text, duration))) = task_result {
            println!(
                "\n{}",
                format!("--- MODEL: {} ({:?}) ---", name, duration)
                    .bright_green()
                    .bold()
            );
            println!("{}", response_text);
            println!("{}", "-".repeat(50).bright_black());
        }
    }

    Ok(())
}
