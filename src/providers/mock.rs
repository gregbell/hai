use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

use super::Provider;
use crate::config::Config;

pub struct MockProvider {
    responses: HashMap<String, String>,
}

impl MockProvider {
    pub fn new(_config: Config) -> Self {
        let mut responses = HashMap::new();
        responses.insert("list all files".to_string(), "ls -la".to_string());
        responses.insert(
            "find all text files".to_string(),
            "find . -name \"*.txt\"".to_string(),
        );
        responses.insert(
            "count lines in all python files".to_string(),
            "find . -name \"*.py\" | xargs wc -l".to_string(),
        );
        Self { responses }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn get_command_suggestion(&self, prompt: &str, _system_prompt: String) -> Result<String> {
        // Try to find an exact match
        if let Some(response) = self.responses.get(prompt) {
            return Ok(response.clone());
        }

        // If no exact match, return a default response
        Ok("echo \"Command not found for this prompt\"".to_string())
    }
}
