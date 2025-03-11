use anyhow::{anyhow, Result};
use async_trait::async_trait;

mod anthropic;
#[cfg(test)]
mod mock;
mod openai;

pub use anthropic::AnthropicProvider;
#[cfg(test)]
pub use mock::MockProvider;
pub use openai::OpenAIProvider;

use crate::config::Config;

#[async_trait]
pub trait Provider: Send + Sync {
    async fn get_command_suggestion(&self, prompt: &str, system_prompt: String) -> Result<String>;
}

pub fn create_provider(model_name: &str, config: &Config) -> Result<Box<dyn Provider>> {
    let model_config = config
        .models()
        .and_then(|models| models.get(model_name))
        .ok_or_else(|| anyhow!("Model '{}' not found in config", model_name))?;

    let model = model_config
        .model
        .clone()
        .unwrap_or_else(|| model_name.to_string());
    let auth_token = config.get_provider_auth_token(&model_config.provider, model_config);

    match model_config.provider.as_str() {
        "openai" => Ok(Box::new(OpenAIProvider::new(
            model,
            auth_token,
            config.clone(),
        ))),
        "anthropic" => Ok(Box::new(AnthropicProvider::new(
            model,
            auth_token,
            config.clone(),
        ))),
        #[cfg(test)]
        "mock" => Ok(Box::new(MockProvider::new(config.clone()))),
        _ => Err(anyhow!("Unknown provider: {}", model_config.provider)),
    }
}
