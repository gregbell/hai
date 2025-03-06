use anyhow::{Context, Result};
use async_trait::async_trait;

mod openai;
mod anthropic;
#[cfg(test)]
mod mock;

pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;
#[cfg(test)]
pub use mock::MockProvider;

#[async_trait]
pub trait AIProvider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: &str) -> Result<String>;
}

pub fn create_provider(
    model_name: &str,
    config: &crate::Config,
) -> Result<Box<dyn AIProvider>> {
    let models = config.models.as_ref().context("No models configured")?;
    let model_config = models.get(model_name).context(format!("Model '{}' not found in config", model_name))?;
    
    let temperature = config.temperature.unwrap_or(0.7);
    let max_tokens = config.max_tokens.unwrap_or(100);
    
    // Determine provider based on model name or API URL
    if model_name.starts_with("gpt-") || model_config.api_url.contains("openai.com") {
        Ok(Box::new(OpenAIProvider::new(
            model_config.api_url.clone(),
            model_config.auth_token.clone(),
            model_name.to_string(),
            temperature,
            max_tokens,
        )))
    } else if model_name.starts_with("claude-") || model_config.api_url.contains("anthropic.com") {
        Ok(Box::new(AnthropicProvider::new(
            model_config.api_url.clone(),
            model_config.auth_token.clone(),
            model_name.to_string(),
            temperature,
            max_tokens,
        )))
    } else {
        // Default to OpenAI provider
        Ok(Box::new(OpenAIProvider::new(
            model_config.api_url.clone(),
            model_config.auth_token.clone(),
            model_name.to_string(),
            temperature,
            max_tokens,
        )))
    }
} 