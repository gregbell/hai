use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use serde::Deserialize;
use std::io::{self, Read};
use std::process::Command;

mod providers;
mod error;
mod utils;
mod history;

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

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "default-model")]
    default_model: Option<String>,
    temperature: Option<f32>,
    shell: Option<String>,
    #[serde(rename = "history-size")]
    history_size: Option<usize>,
    #[serde(rename = "system-prompt")]
    system_prompt: Option<String>,
    #[serde(rename = "max-tokens")]
    max_tokens: Option<usize>,
    models: Option<std::collections::HashMap<String, ModelConfig>>,
}

impl Config {
    /// Get the default model name, can be overridden by HAI_DEFAULT_MODEL env var
    pub fn default_model(&self) -> String {
        std::env::var("HAI_DEFAULT_MODEL")
            .ok()
            .unwrap_or_else(|| self.default_model.clone().unwrap_or_else(|| "gpt-4o-mini".to_string()))
    }

    /// Get the temperature value (0.0 to 1.0)
    /// Default: 0.3 - Lower values make responses more deterministic
    pub fn temperature(&self) -> f32 {
        self.temperature.unwrap_or(0.3)
    }

    /// Get the shell to use for command execution
    pub fn shell(&self) -> String {
        std::env::var("SHELL")
            .ok()
            .unwrap_or_else(|| self.shell.clone().unwrap_or_else(|| "bash".to_string()))
    }

    /// Get the maximum number of history entries to keep
    pub fn history_size(&self) -> usize {
        self.history_size.unwrap_or(50)
    }

    /// Get the system prompt for AI, including OS and shell information
    pub fn system_prompt(&self) -> String {
        let base_prompt = self.system_prompt.clone().unwrap_or_else(|| 
            "You are a helpful AI that converts natural language to shell commands. Respond with ONLY the shell command, no explanations or markdown formatting.".to_string()
        );

        // Get OS information
        let os_name = std::env::consts::OS;
        let os_version = get_os_version();

        // Get shell information
        let shell = self.shell();
        let shell_name = std::path::Path::new(&shell)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        format!(
            "{}\nOperating System: {} {}\nShell: {}\nPlease ensure all commands are compatible with this environment.",
            base_prompt, os_name, os_version, shell_name
        )
    }

    /// Get the maximum number of tokens for AI response
    pub fn max_tokens(&self) -> usize {
        self.max_tokens.unwrap_or(100)
    }

    /// Get the auth token for a specific provider, checking environment variables first
    pub fn get_provider_auth_token(&self, provider: &str, model_config: &ModelConfig) -> String {
        match provider {
            "openai" => std::env::var("HAI_OPENAI_TOKEN")
                .ok()
                .unwrap_or_else(|| model_config.auth_token.clone()),
            "anthropic" => std::env::var("HAI_ANTHROPIC_TOKEN")
                .ok()
                .unwrap_or_else(|| model_config.auth_token.clone()),
            _ => model_config.auth_token.clone(),
        }
    }
}

/// Get the OS version in a cross-platform way
fn get_os_version() -> String {
    match std::env::consts::OS {
        "linux" => {
            // Try reading from /etc/os-release first (most Linux distributions)
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                // First try PRETTY_NAME for more descriptive version
                if let Some(version) = content.lines()
                    .find(|line| line.starts_with("PRETTY_NAME="))
                    .map(|line| line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
                {
                    return version;
                }
                
                // Then try VERSION_ID for distributions that have it
                if let Some(version) = content.lines()
                    .find(|line| line.starts_with("VERSION_ID="))
                    .map(|line| line.trim_start_matches("VERSION_ID=").trim_matches('"').to_string())
                {
                    return version;
                }
                
                // For rolling releases like Arch, use NAME
                if let Some(version) = content.lines()
                    .find(|line| line.starts_with("NAME="))
                    .map(|line| line.trim_start_matches("NAME=").trim_matches('"').to_string())
                {
                    return version;
                }
            }
            
            // Fallback to uname if os-release is not available or readable
            if let Ok(output) = std::process::Command::new("uname").arg("-r").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }
            
            "unknown version".to_string()
        },
        "macos" => {
            // Use sw_vers command on macOS
            if let Ok(output) = std::process::Command::new("sw_vers").arg("-productVersion").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }
            
            "unknown version".to_string()
        },
        "windows" => {
            // Use PowerShell to get Windows version
            let args = [
                "-NoProfile",
                "-Command",
                "[System.Environment]::OSVersion.Version.ToString()",
            ];
            
            if let Ok(output) = std::process::Command::new("powershell")
                .args(&args)
                .output()
            {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }
            
            "unknown version".to_string()
        },
        _ => "unknown version".to_string(),
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    provider: String,
    model: Option<String>,
    #[serde(rename = "auth-token")]
    auth_token: String,
}

fn load_config() -> Result<Config> {
    let config_dir = utils::ensure_config_dir()?;
    let config_path = config_dir.join("config.toml");
    
    if !config_path.exists() {
        // Guide the user through initial setup
        utils::guide_initial_setup()?;
        
        // If we still don't have a config file, create a default one
        if !config_path.exists() {
            utils::create_default_config_if_not_exists()?;
        }
    }
    
    let config_str = std::fs::read_to_string(config_path)
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&config_str)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

fn get_prompt_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

async fn get_command_suggestion(prompt: &str, config: &Config, model_name: &str) -> Result<String> {
    let provider = providers::create_provider(model_name, config)?;
    provider.get_command_suggestion(prompt, config.system_prompt()).await
}

fn execute_command(command: &str, shell: &str) -> Result<()> {
    let status = Command::new(shell)
        .arg("-c")
        .arg(command)
        .status()
        .context("Failed to execute command")?;
    
    if !status.success() {
        return Err(anyhow::anyhow!("Command exited with non-zero status: {}", status));
    }
    
    Ok(())
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;
    
    // Handle history command
    if cli.show_history {
        let history = history::History::load()?;
        println!("Command History:");
        println!("{:<10} {:<30} {:<50} {}", "Date", "Prompt", "Command", "Executed");
        println!("{:-<100}", "");
        
        for entry in history.get_entries().iter().rev() {
            let date = entry.timestamp.format("%Y-%m-%d").to_string();
            let prompt = if entry.prompt.len() > 27 {
                format!("{}...", &entry.prompt[..27])
            } else {
                entry.prompt.clone()
            };
            
            let command = if entry.command.len() > 47 {
                format!("{}...", &entry.command[..47])
            } else {
                entry.command.clone()
            };
            
            println!("{:<10} {:<30} {:<50} {}", date, prompt, command, if entry.executed { "Yes" } else { "No" });
        }
        
        return Ok(());
    }
    
    let prompt = if cli.prompt.is_empty() {
        get_prompt_from_stdin()?
    } else {
        cli.prompt
    };
    
    if prompt.trim().is_empty() {
        return Err(anyhow::anyhow!("No prompt provided"));
    }
    
    let model = cli.model.unwrap_or_else(|| config.default_model());
    let command = get_command_suggestion(&prompt, &config, &model).await?;
    
    println!("Command: {}", command);
    
    // Load history with the configured history size
    let mut history = match history::History::load() {
        Ok(h) => h,
        Err(_) => history::History::new(config.history_size()),
    };
    
    if cli.no_execute {
        // Add to history as not executed
        history.add_entry(&prompt, &command, false);
        history.save()?;
        return Ok(());
    }
    
    let should_execute = if cli.yes {
        true
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Looks good?")
            .default(true)
            .interact()?
    };
    
    if should_execute {
        execute_command(&command, &config.shell())?;
        
        // Add to history as executed
        history.add_entry(&prompt, &command, true);
    } else {
        // Add to history as not executed
        history.add_entry(&prompt, &command, false);
    }
    
    // Save history
    history.save()?;
    
    Ok(())
}

fn main() -> ! {
    error::run_with_error_handling(|| {
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
    use crate::providers::{MockProvider, Provider};
    
    #[tokio::test]
    async fn test_get_command_suggestion() {
        let mock_provider = MockProvider::new();
        let prompt = "list all files";
        let system_prompt = format!(
            "You are a helpful AI that converts natural language to shell commands.\nOperating System: {} {}\nShell: {}\nPlease ensure all commands are compatible with this environment.",
            std::env::consts::OS,
            "test version",
            "bash"
        );
        
        let result = mock_provider.get_command_suggestion(prompt, system_prompt).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ls -la");
    }
    
    #[test]
    fn test_load_config() {
        // This is a basic test that just ensures the function doesn't panic
        // In a real test, we would create a temporary config file and test reading from it
        let result = load_config();
        assert!(result.is_ok());
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

    #[test]
    fn test_config_environment_variables() {
        use std::env;
        
        // Set up a test config
        let config = Config {
            default_model: Some("default-model".to_string()),
            temperature: Some(0.3),
            shell: Some("bash".to_string()),
            history_size: Some(100),
            system_prompt: Some("default prompt".to_string()),
            max_tokens: Some(50),
            models: None,
        };

        // Test HAI_DEFAULT_MODEL override
        env::set_var("HAI_DEFAULT_MODEL", "env-model");
        assert_eq!(config.default_model(), "env-model");
        env::remove_var("HAI_DEFAULT_MODEL");
        assert_eq!(config.default_model(), "default-model");

        // Test default model when none is set
        let config_no_model = Config {
            default_model: None,
            temperature: None,
            shell: None,
            history_size: None,
            system_prompt: None,
            max_tokens: None,
            models: None,
        };
        assert_eq!(config_no_model.default_model(), "gpt-4o-mini");
        assert_eq!(config_no_model.temperature(), 0.3);

        // Test SHELL override
        env::set_var("SHELL", "zsh");
        assert_eq!(config.shell(), "zsh");
        env::remove_var("SHELL");
        assert_eq!(config.shell(), "bash");

        // Test auth token overrides
        let model_config = ModelConfig {
            provider: "openai".to_string(),
            model: None,
            auth_token: "config-token".to_string(),
        };

        env::set_var("HAI_OPENAI_TOKEN", "env-token");
        assert_eq!(config.get_provider_auth_token("openai", &model_config), "env-token");
        env::remove_var("HAI_OPENAI_TOKEN");
        assert_eq!(config.get_provider_auth_token("openai", &model_config), "config-token");
    }

    #[test]
    fn test_os_version() {
        let version = get_os_version();
        assert!(!version.is_empty(), "OS version should not be empty");
        assert_ne!(version, "unknown version", "OS version should be detected");
        
        // Test that the version string doesn't contain any unwanted characters
        assert!(!version.contains('\0'), "Version should not contain null bytes");
        assert!(!version.contains('\n'), "Version should not contain newlines");
        assert!(!version.contains('\r'), "Version should not contain carriage returns");
    }
}
