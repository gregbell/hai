use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::HaiError;
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
        let config = crate::config::load_config()?;

        if !history_path.exists() {
            // Use history_size from config
            return Ok(Self::new(config.history_size()));
        }

        let history_str = fs::read_to_string(&history_path).map_err(|e| {
            HaiError::io(format!(
                "Failed to read history file at {:?}: {}",
                history_path, e
            ))
        })?;

        let mut history: History = serde_json::from_str(&history_str).map_err(|e| {
            HaiError::serialization(format!(
                "Failed to parse history file at {:?}: {}",
                history_path, e
            ))
        })?;

        // Update max_size from config in case it changed
        history.max_size = config.history_size();

        Ok(history)
    }

    pub fn save(&self) -> Result<()> {
        let history_path = get_history_path()?;

        let history_str = serde_json::to_string_pretty(self)
            .map_err(|e| HaiError::serialization(format!("Failed to serialize history: {}", e)))?;

        fs::write(&history_path, history_str).map_err(|e| {
            HaiError::io(format!(
                "Failed to write history file to {:?}: {}",
                history_path, e
            ))
        })?;

        Ok(())
    }
}

fn get_history_path() -> Result<PathBuf> {
    utils::ensure_config_dir()
        .map(|dir| dir.join("history.json"))
        .map_err(|e| HaiError::config(format!("Failed to ensure config directory: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_history_new() {
        // Test with zero max size
        let history = History::new(0);
        assert_eq!(history.entries.len(), 0);
        assert_eq!(history.max_size, 0);

        // Test with non-zero max size
        let history = History::new(100);
        assert_eq!(history.entries.len(), 0);
        assert_eq!(history.max_size, 100);
    }

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

    #[test]
    fn test_get_entries() {
        let mut history = History::new(10);

        // Empty history
        assert_eq!(history.get_entries().len(), 0);

        // Add some entries
        history.add_entry("cmd1", "echo 1", true, "model1");
        history.add_entry("cmd2", "echo 2", false, "model2");

        // Check entries
        let entries = history.get_entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].prompt, "cmd1");
        assert_eq!(entries[1].prompt, "cmd2");
    }

    #[test]
    fn test_save_and_load() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let test_history_path = temp_dir.path().join("history.json");

        // Create a history object
        let mut history = History::new(5);
        history.add_entry("test prompt", "test command", true, "test model");

        // Manually save to our test location
        let history_str = serde_json::to_string_pretty(&history).unwrap();
        fs::write(&test_history_path, history_str).unwrap();

        // Manually load from our test location
        let history_str = fs::read_to_string(&test_history_path).unwrap();
        let loaded_history: History = serde_json::from_str(&history_str).unwrap();

        // Verify loaded history
        assert_eq!(loaded_history.max_size, 5);
        assert_eq!(loaded_history.entries.len(), 1);
        assert_eq!(loaded_history.entries[0].prompt, "test prompt");
        assert_eq!(loaded_history.entries[0].command, "test command");
        assert_eq!(loaded_history.entries[0].executed, true);
        assert_eq!(loaded_history.entries[0].model, "test model");
    }

    #[test]
    fn test_history_max_size_edge_cases() {
        // Test with max_size of 1
        let mut history = History::new(1);
        history.add_entry("cmd1", "echo 1", true, "model1");
        assert_eq!(history.entries.len(), 1);

        history.add_entry("cmd2", "echo 2", false, "model2");
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].prompt, "cmd2");

        // Test with max_size of 0 (edge case)
        let mut history = History::new(0);
        history.add_entry("cmd1", "echo 1", true, "model1");
        assert_eq!(
            history.entries.len(),
            0,
            "With max_size=0, no entries should be stored"
        );
    }

    #[test]
    fn test_empty_strings() {
        let mut history = History::new(5);

        // Test with empty strings
        history.add_entry("", "", true, "");
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries[0].prompt, "");
        assert_eq!(history.entries[0].command, "");
        assert_eq!(history.entries[0].model, "");
    }

    #[test]
    fn test_timestamp_is_set() {
        let mut history = History::new(5);

        // Get current time before adding entry
        let before = chrono::Utc::now();

        // Add entry
        history.add_entry("test", "test", true, "test");

        // Get current time after adding entry
        let after = chrono::Utc::now();

        // Verify timestamp is between before and after
        let entry_time = history.entries[0].timestamp;
        assert!(
            entry_time >= before,
            "Entry timestamp should be after or equal to 'before' time"
        );
        assert!(
            entry_time <= after,
            "Entry timestamp should be before or equal to 'after' time"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let mut history = History::new(10);
        history.add_entry("prompt1", "command1", true, "model1");
        history.add_entry("prompt2", "command2", false, "model2");

        // Serialize
        let serialized = serde_json::to_string(&history).unwrap();

        // Deserialize
        let deserialized: History = serde_json::from_str(&serialized).unwrap();

        // Verify
        assert_eq!(deserialized.max_size, 10);
        assert_eq!(deserialized.entries.len(), 2);
        assert_eq!(deserialized.entries[0].prompt, "prompt1");
        assert_eq!(deserialized.entries[0].command, "command1");
        assert_eq!(deserialized.entries[0].executed, true);
        assert_eq!(deserialized.entries[0].model, "model1");
        assert_eq!(deserialized.entries[1].prompt, "prompt2");
        assert_eq!(deserialized.entries[1].command, "command2");
        assert_eq!(deserialized.entries[1].executed, false);
        assert_eq!(deserialized.entries[1].model, "model2");
    }
}
