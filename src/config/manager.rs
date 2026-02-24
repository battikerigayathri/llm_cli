use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(dead_code)]
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
    pub providers: ProviderConfigs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfigs {
    pub openai: ProviderConfig,
    pub anthropic: ProviderConfig,
    pub google: ProviderConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub api_key_env: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub default: String,
    pub available: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub provider: String,
    pub display_name: String,
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
                providers: ProviderConfigs {
                    openai: ProviderConfig {
                        api_key: None,
                        api_key_env: "OPENAI_API_KEY".to_string(),
                        enabled: true,
                    },
                    anthropic: ProviderConfig {
                        api_key: None,
                        api_key_env: "ANTHROPIC_API_KEY".to_string(),
                        enabled: false,
                    },
                    google: ProviderConfig {
                        api_key: None,
                        api_key_env: "GOOGLE_API_KEY".to_string(),
                        enabled: true,
                    },
                },
            },
            models: ModelConfig {
                // Setting GPT-4o as the new default
                default: "gpt-4o".to_string(),
                available: vec![
                    ModelInfo {
                        name: "gpt-4o".to_string(),
                        provider: "openai".to_string(),
                        display_name: "GPT-4o".to_string(),
                    },
                    ModelInfo {
                        name: "gpt-4".to_string(),
                        provider: "openai".to_string(),
                        display_name: "GPT-4".to_string(),
                    },
                    ModelInfo {
                        name: "gpt-3.5-turbo".to_string(),
                        provider: "openai".to_string(),
                        display_name: "GPT-3.5 Turbo".to_string(),
                    },
                    ModelInfo {
                        name: "claude-3-5-sonnet-20241022".to_string(),
                        provider: "anthropic".to_string(),
                        display_name: "Claude 3.5 Sonnet".to_string(),
                    },
                    ModelInfo {
                        name: "claude-3-haiku-20240307".to_string(),
                        provider: "anthropic".to_string(),
                        display_name: "Claude 3 Haiku".to_string(),
                    },
                    ModelInfo {
                        name: "gemini-pro".to_string(),
                        provider: "google".to_string(),
                        display_name: "Gemini Pro".to_string(),
                    },
                ],
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
        // This forces the file to appear in your current project folder
        // exactly where you are running the 'cargo run' command.
        Ok(PathBuf::from("config.toml"))
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
            ["api", "providers", provider, "api_key"] => match *provider {
                "openai" => self.config.api.providers.openai.api_key = Some(value.to_string()),
                "anthropic" => {
                    self.config.api.providers.anthropic.api_key = Some(value.to_string())
                }
                "google" => self.config.api.providers.google.api_key = Some(value.to_string()),
                _ => anyhow::bail!("Unknown provider: {}", provider),
            },
            ["api", "providers", provider, "enabled"] => {
                let enabled: bool = value.parse()?;
                match *provider {
                    "openai" => self.config.api.providers.openai.enabled = enabled,
                    "anthropic" => self.config.api.providers.anthropic.enabled = enabled,
                    "google" => self.config.api.providers.google.enabled = enabled,
                    _ => anyhow::bail!("Unknown provider: {}", provider),
                }
            }
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

    pub fn get_api_key(&self, provider: &str) -> Result<String> {
        let provider_config = match provider {
            "openai" => &self.config.api.providers.openai,
            "anthropic" => &self.config.api.providers.anthropic,
            "google" => &self.config.api.providers.google,
            _ => anyhow::bail!("Unknown provider: {}", provider),
        };

        if let Some(key) = &provider_config.api_key {
            return Ok(key.clone());
        }

        std::env::var(&provider_config.api_key_env).map_err(|_| {
            anyhow::anyhow!(
                "API key not found for {}. Set {} or configure api.providers.{}.api_key",
                provider,
                provider_config.api_key_env,
                provider
            )
        })
    }

    pub fn get_model_info(&self, model_name: &str) -> Option<&ModelInfo> {
        // Access self.config first, then .models
        self.config
            .models
            .available
            .iter()
            .find(|m| m.name == model_name)
    }
    pub fn get_available_models(&self) -> Vec<&ModelInfo> {
        self.config
            .models
            .available
            .iter()
            .filter(|m| match m.provider.as_str() {
                "openai" => self.config.api.providers.openai.enabled,
                "anthropic" => self.config.api.providers.anthropic.enabled,
                "google" => self.config.api.providers.google.enabled,
                _ => false,
            })
            .collect()
    }

    pub fn reset(&mut self) -> Result<()> {
        self.config = Config::default();
        self.save()?;
        Ok(())
    }
}
