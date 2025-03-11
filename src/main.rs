use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::{self, Read};
use std::process::Command;

mod config;
mod error;
mod history;
mod providers;
mod utils;

use config::{load_config, Config};
use error::run_with_error_handling;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The prompt to convert to a shell command
    #[arg(default_value = "")]
    prompt: String,

    /// Skip the prompt and just run the command
    #[arg(short = 'y', long)]
    yes: bool,

    /// Show the command, but don't run it
    #[arg(short = 'n', long = "no-execute")]
    no_execute: bool,

    /// Select the model to use
    #[arg(short = 'm', long)]
    model: Option<String>,

    /// Show command history
    #[arg(short = 'H', long = "history")]
    show_history: bool,
}

fn get_prompt_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

async fn get_command_suggestion(prompt: &str, config: &Config) -> Result<String> {
    // Get the provider name
    let provider_name = std::env::var("HAI_DEFAULT_MODEL")
        .ok()
        .unwrap_or_else(|| config.default_model());

    let provider = providers::create_provider(&provider_name, config)?;
    provider
        .get_command_suggestion(prompt, config.system_prompt())
        .await
}

fn execute_command(command: &str, shell: &str) -> Result<()> {
    // Run the command using the specified shell
    let status = Command::new(shell)
        .arg("-c")
        .arg(command)
        .status()
        .context("Failed to execute command")?;

    // If command failed, return an error
    if !status.success() {
        return Err(anyhow::anyhow!(
            "Command '{}' failed with exit code: {}",
            command,
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // If showing history, do that first and exit
    if cli.show_history {
        let history = history::History::load()?;
        println!("Command History:");
        println!("---------------");
        for (i, entry) in history.get_entries().iter().enumerate() {
            println!("{}: '{}'", i + 1, entry.prompt);
            println!("   $ {}", entry.command);
            println!(
                "   [{}] [Model: {}] [{}]",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.model,
                if entry.executed {
                    "Executed"
                } else {
                    "Not executed"
                }
            );
            println!();
        }
        return Ok(());
    }

    // Get the input prompt either from CLI arguments or stdin
    let prompt = if cli.prompt.is_empty() {
        get_prompt_from_stdin()?
    } else {
        cli.prompt.clone()
    };

    // If we have no prompt, just exit
    if prompt.trim().is_empty() {
        return Err(anyhow::anyhow!("No prompt provided"));
    }

    // Load the config
    let config = load_config()?;

    // Get the model to use, prioritizing:
    // 1. CLI --model flag
    // 2. HAI_DEFAULT_MODEL env var
    // 3. config file default-model field
    let model_name = cli
        .model
        .or_else(|| std::env::var("HAI_DEFAULT_MODEL").ok())
        .unwrap_or_else(|| config.default_model());

    // Get a command suggestion
    let command = get_command_suggestion(&prompt, &config).await?;

    // Skip confirmation and run the command if --yes flag is set
    if cli.yes {
        println!("$ {}", command);
        if !cli.no_execute {
            execute_command(&command, &config.shell())?;
        }
    } else {
        // Show the command and ask for confirmation
        println!("Suggested command:");
        println!("$ {}", command);

        if !cli.no_execute {
            // Ask for confirmation
            let confirmation = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Run this command?")
                .default(true)
                .interact()?;

            if confirmation {
                execute_command(&command, &config.shell())?;
            }
        }
    }

    // Load history
    let mut history = history::History::load()?;

    // Add the prompt and command to history
    history.add_entry(&prompt, &command, false, &model_name);

    // Save history
    history.save()?;

    Ok(())
}

fn main() -> ! {
    run_with_error_handling(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(run())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::ModelConfig;
    use std::collections::HashMap;
    use std::env;

    #[tokio::test]
    async fn test_get_command_suggestion() {
        // Set up a mock config for testing
        let mut config = Config::default();
        let mut models = HashMap::new();
        models.insert(
            "mock".to_string(),
            ModelConfig {
                provider: "mock".to_string(),
                model: None,
                auth_token: "test-token".to_string(),
            },
        );
        config.set_models(models);

        // Set the environment variable for testing
        env::set_var("HAI_DEFAULT_MODEL", "mock");

        // Test with a known prompt
        let prompt = "list all files";
        let result = get_command_suggestion(prompt, &config).await;

        // Clean up
        env::remove_var("HAI_DEFAULT_MODEL");

        assert!(result.is_ok());
        if let Ok(command) = result {
            assert_eq!(command, "ls -la");
        }
    }

    #[test]
    fn test_get_prompt_from_stdin() {
        // This is hard to test without mocking stdin, so we'll skip for now
    }

    #[test]
    fn test_execute_command() {
        // Test with a simple command that should always succeed
        let result = execute_command("echo test", "bash");
        assert!(result.is_ok());

        // Test with a command that should fail
        let result = execute_command("exit 1", "bash");
        assert!(result.is_err());
    }
}
