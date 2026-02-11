use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "llm-cli")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool for interacting with LLMs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ask a one-shot question to the LLM
    Ask {
        /// The question to ask
        #[arg(required_unless_present = "file")]
        query: Option<String>,
        
        /// Read query from file
        #[arg(short, long)]
        file: Option<String>,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Model to use (overrides config)
        #[arg(short, long)]
        model: Option<String>,
        
        /// Use a template
        #[arg(short, long)]
        template: Option<String>,
    },
    
    /// Start an interactive chat session
    Chat {
        /// Session name (creates new or loads existing)
        #[arg(short, long)]
        session: Option<String>,
        
        /// Model to use (overrides config)
        #[arg(short, long)]
        model: Option<String>,
    },
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    
    /// Manage chat sessions
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },
    
    /// Manage prompt templates
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., api.provider, models.default)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Show all configuration
    Show,
    /// Reset configuration to defaults
    Reset,
}

#[derive(Subcommand)]
pub enum SessionAction {
    /// List all sessions
    List,
    /// Show session details
    Show {
        /// Session name
        name: String,
    },
    /// Delete a session
    Delete {
        /// Session name
        name: String,
    },
    /// Export a session to file
    Export {
        /// Session name
        name: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Subcommand)]
pub enum TemplateAction {
    /// Create a new template
    Create {
        /// Template name
        name: String,
    },
    /// List all templates
    List,
    /// Show template content
    Show {
        /// Template name
        name: String,
    },
    /// Delete a template
    Delete {
        /// Template name
        name: String,
    },
}