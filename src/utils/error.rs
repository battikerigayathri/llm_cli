use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmCliError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Session error: {0}")]
    SessionError(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, LlmCliError>;