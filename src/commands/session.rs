use crate::cli::SessionAction;
use crate::session::SessionStore;
use anyhow::Result;
use colored::*;

pub fn execute(action: SessionAction) -> Result<()> {
    let store = SessionStore::new()?;
    
    match action {
        SessionAction::List => {
            let sessions = store.list_sessions()?;
            if sessions.is_empty() {
                println!("{}", "No sessions found".yellow());
            } else {
                println!("{}", "Sessions:".green().bold());
                for session in sessions {
                    println!("  • {}", session.cyan());
                }
            }
        }
        SessionAction::Show { name } => {
            match store.load_session(&name)? {
                Some(session) => {
                    println!("{} {}", "Session:".green().bold(), name.cyan());
                    println!("Messages: {}", session.messages.len());
                    println!("Created: {}", chrono::DateTime::from_timestamp(session.created_at, 0)
                        .map(|dt| dt.to_rfc2822())
                        .unwrap_or_else(|| "Unknown".to_string()));
                }
                None => {
                    println!("{} Session '{}' not found", "✗".red(), name);
                }
            }
        }
        SessionAction::Delete { name } => {
            store.delete_session(&name)?;
            println!("{} Deleted session '{}'", "✓".green(), name);
        }
        SessionAction::Export { name, output } => {
            match store.load_session(&name)? {
                Some(session) => {
                    let json = serde_json::to_string_pretty(&session)?;
                    std::fs::write(&output, json)?;
                    println!("{} Exported session to {}", "✓".green(), output.cyan());
                }
                None => {
                    println!("{} Session '{}' not found", "✗".red(), name);
                }
            }
        }
    }
    
    Ok(())
}