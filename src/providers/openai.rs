use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: usize,
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

#[async_trait]
impl Provider for OpenAIProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: &str) -> Result<String> {
        let client = reqwest::Client::new();
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.auth_token))?,
        );
        
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
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
            .context("Failed to send request to OpenAI API")?;
        
        let response_body: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI API response")?;
        
        let command = response_body
            .choices
            .first()
            .context("No choices in response")?
            .message
            .content
            .trim()
            .to_string();
        
        Ok(command)
    }
} 