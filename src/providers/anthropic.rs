use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Provider;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    system: String,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ResponseContent>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    text: String,
}

pub struct AnthropicProvider {
    client: Client,
    model: String,
    auth_token: String,
}

impl AnthropicProvider {
    pub fn new(model: String, auth_token: String) -> Self {
        Self {
            client: Client::new(),
            model,
            auth_token,
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 100,
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            system: system_prompt,
        };

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.auth_token)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Anthropic API error: {}",
                error_text
            ));
        }

        let response: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic API response")?;

        Ok(response.content[0].text.clone())
    }
} 