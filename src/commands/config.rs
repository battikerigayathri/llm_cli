use crate::cli::ConfigAction;
use crate::config::ConfigManager;
use anyhow::Result;
use colored::*;

pub fn execute(action: ConfigAction) -> Result<()> {
    let mut config_manager = ConfigManager::new()?;
    
    match action {
        ConfigAction::Set { key, value } => {
            config_manager.set(&key, &value)?;
            println!("{} Set {} = {}", "✓".green(), key.cyan(), value);
        }
        ConfigAction::Get { key } => {
            // TODO: Implement get by key path
            println!("{} Getting key: {}", "→".blue(), key);
        }
        ConfigAction::Show => {
            let config = config_manager.get();
            println!("{}", "Current Configuration:".green().bold());
            println!("{}", toml::to_string_pretty(config)?);
        }
        ConfigAction::Reset => {
            config_manager.reset()?;
            println!("{} Configuration reset to defaults", "✓".green());
        }
    }
    
    Ok(())
}