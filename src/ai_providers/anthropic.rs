use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::AIProvider;

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

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    temperature: f32,
    max_tokens: usize,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.auth_token)?,
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
        );
        
        let request = AnthropicRequest {
            model: self.model.clone(),
            messages: vec![
                AnthropicMessage {
                    role: "system".to_string(),
                    content: vec![AnthropicContent {
                        content_type: "text".to_string(),
                        text: system_prompt.to_string(),
                    }],
                },
                AnthropicMessage {
                    role: "user".to_string(),
                    content: vec![AnthropicContent {
                        content_type: "text".to_string(),
                        text: prompt.to_string(),
                    }],
                },
            ],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };
        
        let response = client
            .post(&self.api_url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;
        
        let response_body: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic API response")?;
        
        let command = response_body
            .content
            .first()
            .context("No content in response")?
            .text
            .trim()
            .to_string();
        
        Ok(command)
    }
} 