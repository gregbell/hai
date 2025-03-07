use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json;

use super::Provider;

pub struct OpenAIProvider {
    api_url: String,
    auth_token: String,
    model: String,
    temperature: f32,
    max_tokens: usize,
}

impl OpenAIProvider {
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

#[async_trait]
impl Provider for OpenAIProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String> {
        let client = reqwest::Client::new();
        
        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": system_prompt
            }),
            serde_json::json!({
                "role": "user",
                "content": prompt
            })
        ];
        
        let response = client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&serde_json::json!({
                "model": self.model,
                "messages": messages,
                "temperature": self.temperature,
                "max_tokens": self.max_tokens,
            }))
            .send()
            .await?;
        
        let response_json: OpenAIResponse = response.json().await?;
        
        if let Some(choice) = response_json.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No choices in response"))
        }
    }
} 