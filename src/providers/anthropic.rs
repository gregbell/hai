use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json;

use super::Provider;

pub struct AnthropicProvider {
    api_url: String,
    auth_token: String,
    model: String,
    temperature: f32,
    max_tokens: usize,
}

impl AnthropicProvider {
    pub fn new(
        api_url: String,
        auth_token: String,
        model: String,
        temperature: f32,
        max_tokens: usize,
    ) -> Self {
        Self {
            api_url,
            auth_token,
            model,
            temperature,
            max_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    text: String,
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String> {
        let client = reqwest::Client::new();
        
        let request_body = serde_json::json!({
            "model": self.model,
            "prompt": format!("{}\n\nHuman: {}\n\nAssistant:", system_prompt, prompt),
            "temperature": self.temperature,
            "max_tokens_to_sample": self.max_tokens,
        });
        
        let response = client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.auth_token)
            .json(&request_body)
            .send()
            .await?;
        
        let response_json: AnthropicResponse = response.json().await?;
        
        if let Some(content) = response_json.content.first() {
            Ok(content.text.clone())
        } else {
            Err(anyhow::anyhow!("No content in response"))
        }
    }
} 