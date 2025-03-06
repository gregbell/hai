use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::path::PathBuf;

/// Ensures that the config directory exists
pub fn ensure_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("hai");
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
    }
    
    Ok(config_dir)
}

/// Creates a default config file if it doesn't exist
pub fn create_default_config_if_not_exists() -> Result<()> {
    let config_dir = ensure_config_dir()?;
    let config_path = config_dir.join("config.toml");
    
    if !config_path.exists() {
        // Create a minimal default config
        let default_config = r#"# Default model to use if --model is not specified
default-model = "gpt-4o"

# Global settings
temperature = 0.7
confirm-by-default = false
shell = "bash"
system-prompt = "You are a helpful AI that converts natural language to shell commands. Respond with ONLY the shell command, no explanations or markdown formatting."
max-tokens = 100
history-size = 50

# Add your models here
[models.gpt-4o]
api-url = "https://api.openai.com/v1/chat/completions"
auth-token = "your-openai-api-key"
"#;
        
        fs::write(&config_path, default_config)
            .context("Failed to write default config")?;
        
        println!("Created default config file at {}", config_path.display());
        println!("Please edit this file to add your API keys");
    }
    
    Ok(())
}

/// Guides the user through initial setup
pub fn guide_initial_setup() -> Result<()> {
    let config_dir = ensure_config_dir()?;
    let config_path = config_dir.join("config.toml");
    
    if config_path.exists() {
        return Ok(());
    }
    
    println!("Welcome to hai! Let's set up your configuration.");
    println!("You'll need an API key from OpenAI or Anthropic to use hai.");
    
    let provider_options = vec!["OpenAI", "Anthropic", "Skip for now"];
    let provider_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which AI provider would you like to use?")
        .default(0)
        .items(&provider_options)
        .interact()?;
    
    let mut config = String::from(r#"# Default model to use if --model is not specified
temperature = 0.7
confirm-by-default = false
shell = "bash"
system-prompt = "You are a helpful AI that converts natural language to shell commands. Respond with ONLY the shell command, no explanations or markdown formatting."
max-tokens = 100
history-size = 50

"#);
    
    match provider_selection {
        0 => {
            // OpenAI
            config.push_str("default-model = \"gpt-4o\"\n\n");
            config.push_str("# OpenAI configuration\n");
            config.push_str("[models.gpt-4o]\n");
            config.push_str("api-url = \"https://api.openai.com/v1/chat/completions\"\n");
            
            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your OpenAI API key (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;
            
            if api_key.is_empty() {
                config.push_str("auth-token = \"your-openai-api-key\"\n");
                println!("You'll need to edit the config file later to add your API key.");
            } else {
                config.push_str(&format!("auth-token = \"{}\"\n", api_key));
            }
        },
        1 => {
            // Anthropic
            config.push_str("default-model = \"claude-3\"\n\n");
            config.push_str("# Anthropic configuration\n");
            config.push_str("[models.claude-3]\n");
            config.push_str("api-url = \"https://api.anthropic.com/v1/complete\"\n");
            
            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Anthropic API key (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;
            
            if api_key.is_empty() {
                config.push_str("auth-token = \"your-anthropic-api-key\"\n");
                println!("You'll need to edit the config file later to add your API key.");
            } else {
                config.push_str(&format!("auth-token = \"{}\"\n", api_key));
            }
        },
        _ => {
            // Skip
            config.push_str("default-model = \"gpt-4o\"\n\n");
            config.push_str("# OpenAI configuration\n");
            config.push_str("[models.gpt-4o]\n");
            config.push_str("api-url = \"https://api.openai.com/v1/chat/completions\"\n");
            config.push_str("auth-token = \"your-openai-api-key\"\n");
            println!("You'll need to edit the config file later to add your API key.");
        }
    }
    
    fs::write(&config_path, config)
        .context("Failed to write config file")?;
    
    println!("\nConfiguration file created at: {}", config_path.display());
    println!("You can edit this file anytime to change your settings.");
    
    Ok(())
} 