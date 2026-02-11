use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub models: ModelConfig,
    pub chat: ChatConfig,
    pub session: SessionConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub api_key_env: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub default: String,
    pub fallback: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatConfig {
    pub temperature: f32,
    pub max_tokens: u32,
    pub streaming: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionConfig {
    pub auto_save: bool,
    pub max_history: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub syntax_highlighting: bool,
    pub markdown_rendering: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                provider: "anthropic".to_string(),
                api_key: None,
                api_key_env: "ANTHROPIC_API_KEY".to_string(),
            },
            models: ModelConfig {
                default: "claude-sonnet-4".to_string(),
                fallback: Some("claude-haiku-4".to_string()),
            },
            chat: ChatConfig {
                temperature: 0.7,
                max_tokens: 4096,
                streaming: true,
            },
            session: SessionConfig {
                auto_save: true,
                max_history: 50,
            },
            output: OutputConfig {
                syntax_highlighting: true,
                markdown_rendering: true,
            },
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create(&config_path)?;
        
        Ok(Self {
            config_path,
            config,
        })
    }
    
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "llm-cli", "llm-cli")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;
        
        Ok(config_dir.join("config.toml"))
    }
    
    fn load_or_create(path: &PathBuf) -> Result<Config> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            let content = toml::to_string_pretty(&config)?;
            std::fs::write(path, content)?;
            Ok(config)
        }
    }
    
    pub fn get(&self) -> &Config {
        &self.config
    }
    
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();
        
        match parts.as_slice() {
            ["api", "provider"] => self.config.api.provider = value.to_string(),
            ["api", "api_key_env"] => self.config.api.api_key_env = value.to_string(), 
            ["models", "default"] => self.config.models.default = value.to_string(),
            ["chat", "temperature"] => self.config.chat.temperature = value.parse()?,
            ["chat", "max_tokens"] => self.config.chat.max_tokens = value.parse()?,
            ["chat", "streaming"] => self.config.chat.streaming = value.parse()?,
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        
        self.save()?;
        Ok(())
    }
    
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
    
    pub fn get_api_key(&self) -> Result<String> {
        if let Some(key) = &self.config.api.api_key {
            return Ok(key.clone());
        }
        
        std::env::var(&self.config.api.api_key_env)
            .map_err(|_| anyhow::anyhow!(
                "API key not found. Set {} or configure api.api_key", 
                self.config.api.api_key_env
            ))
    }
    
    pub fn reset(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save()?;
        Ok(())
    }
}