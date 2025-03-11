use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Provider;
use crate::config::Config;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct OpenAIProvider {
    client: Client,
    model: String,
    auth_token: String,
    config: Config,
}

impl OpenAIProvider {
    pub fn new(model: String, auth_token: String, config: Config) -> Self {
        Self {
            client: Client::new(),
            model,
            auth_token,
            config,
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: self.config.temperature(),
            max_tokens: self.config.max_tokens() as u32,
        };

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let response: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI API response")?;

        Ok(response.choices[0].message.content.clone())
    }
}
