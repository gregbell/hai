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
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    Ok(config_dir)
}

/// Returns the base configuration template
fn get_base_config() -> String {
    format!(
        r#"# Global settings
history-size = 50

# Model configurations
"#
    )
}

/// Creates a default config file if it doesn't exist
pub fn create_default_config_if_not_exists() -> Result<()> {
    let config_dir = ensure_config_dir()?;
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        let mut config = get_base_config();
        config.push_str(
            r#"default-model = "gpt-4o-mini"

[models.gpt-4o-mini]
provider = "openai"
model = "gpt-4o-mini"  # OpenAI's base GPT-4 model
auth-token = ""  # Add your OpenAI API key here

[models.claude-3]
provider = "anthropic"
model = "claude-3-7-sonnet-20250219"  # Anthropic's Claude 3 model
auth-token = ""  # Add your Anthropic API key here
"#,
        );

        std::fs::write(config_path, config)?;
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

    let mut config = get_base_config();

    match provider_selection {
        0 => {
            // OpenAI
            config.push_str("default-model = \"gpt-4o-mini\"\n\n");
            config.push_str("# OpenAI configuration\n");
            config.push_str("[models.gpt-4o-mini]\n");
            config.push_str("provider = \"openai\"\n");
            config.push_str("model = \"gpt-4o-mini\"\n");

            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your OpenAI API key (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;

            if api_key.is_empty() {
                config.push_str("auth-token = \"\"\n");
                println!("You'll need to edit the config file later to add your API key.");
            } else {
                config.push_str(&format!("auth-token = \"{}\"\n", api_key));
            }
        }
        1 => {
            // Anthropic
            config.push_str("default-model = \"claude-3\"\n\n");
            config.push_str("# Anthropic configuration\n");
            config.push_str("[models.claude-3]\n");
            config.push_str("provider = \"anthropic\"\n");
            config.push_str("model = \"claude-3-7-sonnet-20250219\"\n");

            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Anthropic API key (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;

            if api_key.is_empty() {
                config.push_str("auth-token = \"\"\n");
                println!("You'll need to edit the config file later to add your API key.");
            } else {
                config.push_str(&format!("auth-token = \"{}\"\n", api_key));
            }
        }
        _ => {
            // Skip
            config.push_str("default-model = \"gpt-4o-mini\"\n\n");
            config.push_str("# OpenAI configuration\n");
            config.push_str("[models.gpt-4o-mini]\n");
            config.push_str("provider = \"openai\"\n");
            config.push_str("model = \"gpt-4o-mini\"\n");
            config.push_str("auth-token = \"\"\n");
            println!("You'll need to edit the config file later to add your API key.");
        }
    }

    fs::write(&config_path, config).context("Failed to write config file")?;

    println!("\nConfiguration file created at: {}", config_path.display());
    println!("You can edit this file anytime to change your settings.");

    Ok(())
}
