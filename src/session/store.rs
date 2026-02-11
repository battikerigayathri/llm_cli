use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMessage {
    pub role: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub messages: Vec<SessionMessage>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct SessionStore {
    db: Db,
}

impl SessionStore {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }
    
    fn get_db_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "llm-cli", "llm-cli")
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
        
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        
        Ok(data_dir.join("sessions"))
    }
    
    pub fn save_session(&self, session: &Session) -> Result<()> {
        let key = session.name.as_bytes();
        let value = serde_json::to_vec(session)?;
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn load_session(&self, name: &str) -> Result<Option<Session>> {
        let key = name.as_bytes();
        match self.db.get(key)? {
            Some(data) => {
                let session = serde_json::from_slice(&data)?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }
    
    pub fn delete_session(&self, name: &str) -> Result<()> {
        let key = name.as_bytes();
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }
    
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut sessions = Vec::new();
        for item in self.db.iter() {
            let (key, _) = item?;
            let name = String::from_utf8(key.to_vec())?;
            sessions.push(name);
        }
        sessions.sort();
        Ok(sessions)
    }
    
    pub fn add_message(&self, session_name: &str, message: SessionMessage) -> Result<()> {
        let mut session = self.load_session(session_name)?
            .unwrap_or_else(|| Session {
                name: session_name.to_string(),
                messages: Vec::new(),
                created_at: chrono::Utc::now().timestamp(),
                updated_at: chrono::Utc::now().timestamp(),
            });
        
        session.messages.push(message);
        session.updated_at = chrono::Utc::now().timestamp();
        
        self.save_session(&session)?;
        Ok(())
    }
}