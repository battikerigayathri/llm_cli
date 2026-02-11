mod cli;
mod api;
mod config;
mod session;
mod commands;
mod output;
mod template;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Route to appropriate command handler
    match cli.command {
        Commands::Ask { query, file, output, model, template } => {
            commands::ask::execute(query, file, output, model, template).await?;
        }
        Commands::Chat { session, model } => {
            commands::chat::execute(session, model).await?;
        }
        Commands::Config { action } => {
            commands::config::execute(action)?;
        }
        Commands::Session { action } => {
            commands::session::execute(action)?;
        }
        Commands::Template { action } => {
            commands::template::execute(action)?;
        }
    }
    
    Ok(())
}