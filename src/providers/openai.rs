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
    #[serde(default)]
    error: Option<OpenAIError>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
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
        
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
        });

        let response = client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        // Try to parse as error response first
        if !status.is_success() {
            if let Ok(error_response) = serde_json::from_str::<OpenAIResponse>(&response_text) {
                if let Some(error) = error_response.error {
                    return Err(anyhow::anyhow!(
                        "OpenAI API error ({}): {}",
                        error.error_type,
                        error.message
                    ));
                }
            }
            return Err(anyhow::anyhow!(
                "OpenAI API error ({}): {}",
                status,
                response_text
            ));
        }

        // Parse successful response
        let response_json: OpenAIResponse = serde_json::from_str(&response_text)
            .map_err(|e| anyhow::anyhow!("Failed to parse OpenAI response: {}. Response: {}", e, response_text))?;
        
        if let Some(choice) = response_json.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No response choices in OpenAI response: {}", response_text))
        }
    }
} 