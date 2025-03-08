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
    pub model: String,
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

    pub fn add_entry(&mut self, prompt: &str, command: &str, executed: bool, model: &str) {
        let entry = HistoryEntry {
            prompt: prompt.to_string(),
            command: command.to_string(),
            timestamp: chrono::Utc::now(),
            executed,
            model: model.to_string(),
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

        let history_str =
            fs::read_to_string(history_path).context("Failed to read history file")?;

        let history: History =
            serde_json::from_str(&history_str).context("Failed to parse history file")?;

        Ok(history)
    }

    pub fn save(&self) -> Result<()> {
        let history_path = get_history_path()?;

        let history_str =
            serde_json::to_string_pretty(self).context("Failed to serialize history")?;

        fs::write(history_path, history_str).context("Failed to write history file")?;

        Ok(())
    }
}

fn get_history_path() -> Result<PathBuf> {
    let config_dir = utils::ensure_config_dir()?;
    Ok(config_dir.join("history.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_add_entry() {
        let mut history = History::new(2);

        // Add first entry
        history.add_entry("list files", "ls -la", true, "gpt-4o-mini");
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].prompt, "list files");
        assert_eq!(history.entries[0].command, "ls -la");
        assert_eq!(history.entries[0].executed, true);
        assert_eq!(history.entries[0].model, "gpt-4o-mini");

        // Add second entry
        history.add_entry("show processes", "ps aux", false, "claude-3");
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[1].prompt, "show processes");
        assert_eq!(history.entries[1].command, "ps aux");
        assert_eq!(history.entries[1].executed, false);
        assert_eq!(history.entries[1].model, "claude-3");

        // Add third entry (should remove first entry due to max_size)
        history.add_entry("disk space", "df -h", true, "gpt-4o-mini");
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[0].prompt, "show processes");
        assert_eq!(history.entries[1].prompt, "disk space");
    }
}
