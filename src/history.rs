use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub prompt: String,
    pub command: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub executed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    entries: Vec<HistoryEntry>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }
    
    pub fn add_entry(&mut self, prompt: &str, command: &str, executed: bool) {
        let entry = HistoryEntry {
            prompt: prompt.to_string(),
            command: command.to_string(),
            timestamp: chrono::Utc::now(),
            executed,
        };
        
        self.entries.push(entry);
        
        // Trim history if it exceeds max size
        if self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
    }
    
    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
    
    pub fn load() -> Result<Self> {
        let history_path = get_history_path()?;
        
        if !history_path.exists() {
            // Default to 50 entries if not specified
            return Ok(Self::new(50));
        }
        
        let history_str = fs::read_to_string(history_path)
            .context("Failed to read history file")?;
        
        let history: History = serde_json::from_str(&history_str)
            .context("Failed to parse history file")?;
        
        Ok(history)
    }
    
    pub fn save(&self) -> Result<()> {
        let history_path = get_history_path()?;
        
        let history_str = serde_json::to_string_pretty(self)
            .context("Failed to serialize history")?;
        
        fs::write(history_path, history_str)
            .context("Failed to write history file")?;
        
        Ok(())
    }
}

fn get_history_path() -> Result<PathBuf> {
    let config_dir = utils::ensure_config_dir()?;
    Ok(config_dir.join("history.json"))
} 