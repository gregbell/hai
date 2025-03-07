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
pub trait Provider {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String>;
}

pub fn create_provider(
    model_name: &str,
    config: &crate::Config,
) -> Result<Box<dyn Provider>> {
    let models = config.models.as_ref().context("No models configured")?;
    let model_config = models.get(model_name).context(format!("Model '{}' not found in config", model_name))?;
    
    // Determine provider based on the provider field
    match model_config.provider.as_str() {
        "openai" => {
            let api_url = "https://api.openai.com/v1/chat/completions".to_string();
            let model = model_config.model.clone().unwrap_or_else(|| model_name.to_string());
            let auth_token = config.get_provider_auth_token("openai", model_config);
            
            Ok(Box::new(OpenAIProvider::new(
                api_url,
                auth_token,
                model,
                config.temperature(),
                config.max_tokens(),
            )))
        },
        "anthropic" => {
            let api_url = "https://api.anthropic.com/v1/complete".to_string();
            let model = model_config.model.clone().unwrap_or_else(|| model_name.to_string());
            let auth_token = config.get_provider_auth_token("anthropic", model_config);
            
            Ok(Box::new(AnthropicProvider::new(
                api_url,
                auth_token,
                model,
                config.temperature(),
                config.max_tokens(),
            )))
        },
        provider => Err(anyhow::anyhow!("Unsupported provider: {}", provider)),
    }
} 