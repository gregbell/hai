use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

use crate::error::HaiError;
use crate::utils;

// Default system prompt used across the application
pub const DEFAULT_SYSTEM_PROMPT: &str = "
You are Hai, a helpful AI that converts natural language to shell commands. 
Respond with ONLY the shell command, no explanations or markdown formatting.
Make sure commands are compatible with the user's environment and shell.
Your name is Hai. 
If the request from the user is not a clear shell command, respond with a witty but nice message using the \"echo\" command.
Adapt your commands to the specific shell syntax (Bash, Zsh, Fish, PowerShell) that the user is using.
";

#[derive(Debug, Deserialize, Clone)]
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
    models: Option<HashMap<String, ModelConfig>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_model: Some("gpt-4o-mini".to_string()),
            temperature: Some(0.3),
            shell: Some("bash".to_string()),
            history_size: Some(50),
            system_prompt: Some(DEFAULT_SYSTEM_PROMPT.to_string()),
            max_tokens: Some(100),
            models: Some(HashMap::new()),
        }
    }
}

impl Config {
    /// Get the default model name, can be overridden by HAI_DEFAULT_MODEL env var
    pub fn default_model(&self) -> String {
        env::var("HAI_DEFAULT_MODEL").ok().unwrap_or_else(|| {
            self.default_model
                .clone()
                .unwrap_or_else(|| "gpt-4o-mini".to_string())
        })
    }

    /// Get the temperature value (0.0 to 1.0)
    /// Default: 0.3 - Lower values make responses more deterministic
    pub fn temperature(&self) -> f32 {
        self.temperature.unwrap_or(0.3)
    }

    /// Get the shell to use for command execution
    pub fn shell(&self) -> String {
        // First check if the user has explicitly set a shell in the config
        if let Some(shell) = &self.shell {
            return shell.clone();
        }

        // Then check the SHELL environment variable
        if let Ok(shell_path) = env::var("SHELL") {
            let shell_name = std::path::Path::new(&shell_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("bash");

            return shell_name.to_string();
        }

        // On Windows, check if PowerShell is available
        if env::consts::OS == "windows" {
            // Try to detect PowerShell Core (pwsh) first, then fallback to Windows PowerShell
            if Command::new("pwsh")
                .arg("-Command")
                .arg("exit")
                .status()
                .is_ok()
            {
                return "pwsh".to_string();
            }
            if Command::new("powershell")
                .arg("-Command")
                .arg("exit")
                .status()
                .is_ok()
            {
                return "powershell".to_string();
            }
        }

        // Default to bash
        "bash".to_string()
    }

    /// Get the maximum number of history entries to keep
    pub fn history_size(&self) -> usize {
        self.history_size.unwrap_or(50)
    }

    /// Get the system prompt for AI, including OS and shell information
    pub fn system_prompt(&self) -> String {
        let base_prompt = self
            .system_prompt
            .clone()
            .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string());

        // Get OS information
        let os_name = env::consts::OS;
        let os_version = get_os_version();

        // Get shell information
        let shell = self.shell();
        let shell_info = match shell.as_str() {
            "fish" => "Fish shell (fish)",
            "powershell" => "Windows PowerShell",
            "pwsh" => "PowerShell Core (pwsh)",
            "bash" => "Bash shell (bash)",
            "zsh" => "Z shell (zsh)",
            _ => &shell,
        };

        format!(
            "{}\nOperating System: {} {}\nShell: {}\nPlease ensure all commands are compatible with this environment and shell syntax.",
            base_prompt, os_name, os_version, shell_info
        )
    }

    /// Get the maximum number of tokens for AI response
    pub fn max_tokens(&self) -> usize {
        self.max_tokens.unwrap_or(100)
    }

    /// Get the auth token for a specific provider, checking environment variables first
    pub fn get_provider_auth_token(&self, provider: &str, model_config: &ModelConfig) -> String {
        match provider {
            "openai" => env::var("HAI_OPENAI_TOKEN")
                .ok()
                .unwrap_or_else(|| model_config.auth_token.clone()),
            "anthropic" => env::var("HAI_ANTHROPIC_TOKEN")
                .ok()
                .unwrap_or_else(|| model_config.auth_token.clone()),
            _ => model_config.auth_token.clone(),
        }
    }

    /// Get a reference to the model configurations
    pub fn models(&self) -> Option<&HashMap<String, ModelConfig>> {
        self.models.as_ref()
    }

    /// Set the model configurations - only used for testing
    #[cfg(test)]
    pub fn set_models(&mut self, models: HashMap<String, ModelConfig>) {
        self.models = Some(models);
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    pub provider: String,
    pub model: Option<String>,
    #[serde(rename = "auth-token")]
    pub auth_token: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: "mock".to_string(),
            model: None,
            auth_token: String::new(),
        }
    }
}

/// Get the OS version in a cross-platform way
pub fn get_os_version() -> String {
    match env::consts::OS {
        "linux" => {
            // Try reading from /etc/os-release first (most Linux distributions)
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                // First try PRETTY_NAME for more descriptive version
                if let Some(version) = content
                    .lines()
                    .find(|line| line.starts_with("PRETTY_NAME="))
                    .map(|line| {
                        line.trim_start_matches("PRETTY_NAME=")
                            .trim_matches('"')
                            .to_string()
                    })
                {
                    return version;
                }

                // Then try VERSION_ID for distributions that have it
                if let Some(version) = content
                    .lines()
                    .find(|line| line.starts_with("VERSION_ID="))
                    .map(|line| {
                        line.trim_start_matches("VERSION_ID=")
                            .trim_matches('"')
                            .to_string()
                    })
                {
                    return version;
                }

                // For rolling releases like Arch, use NAME
                if let Some(version) =
                    content
                        .lines()
                        .find(|line| line.starts_with("NAME="))
                        .map(|line| {
                            line.trim_start_matches("NAME=")
                                .trim_matches('"')
                                .to_string()
                        })
                {
                    return version;
                }
            }

            // Fallback to uname if os-release is not available or readable
            if let Ok(output) = Command::new("uname").arg("-r").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }

            "unknown version".to_string()
        }
        "macos" => {
            // Use sw_vers command on macOS
            if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }

            "unknown version".to_string()
        }
        "windows" => {
            // Use PowerShell to get Windows version
            let args = [
                "-NoProfile",
                "-Command",
                "[System.Environment]::OSVersion.Version.ToString()",
            ];

            if let Ok(output) = Command::new("powershell").args(&args).output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }

            "unknown version".to_string()
        }
        _ => "unknown version".to_string(),
    }
}

/// Load the configuration from the file system
pub fn load_config() -> Result<Config> {
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

    let config_str = fs::read_to_string(&config_path).map_err(|e| {
        HaiError::io(format!(
            "Failed to read config file at {:?}: {}",
            config_path, e
        ))
    })?;

    let config: Config = toml::from_str(&config_str)
        .map_err(|e| HaiError::config(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_version() {
        let version = get_os_version();
        assert!(!version.is_empty(), "OS version should not be empty");
        assert_ne!(version, "unknown version", "OS version should be detected");

        // Test that the version string doesn't contain any unwanted characters
        assert!(
            !version.contains('\0'),
            "Version should not contain null bytes"
        );
        assert!(
            !version.contains('\n'),
            "Version should not contain newlines"
        );
        assert!(
            !version.contains('\r'),
            "Version should not contain carriage returns"
        );
    }

    #[test]
    fn test_config_environment_variables() {
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

        // Test SHELL override - create a config with no shell set
        let config_no_shell = Config {
            default_model: Some("default-model".to_string()),
            temperature: Some(0.3),
            shell: None,
            history_size: Some(100),
            system_prompt: Some("default prompt".to_string()),
            max_tokens: Some(50),
            models: None,
        };

        // Set SHELL environment variable
        env::set_var("SHELL", "/usr/bin/zsh");
        assert_eq!(config_no_shell.shell(), "zsh");
        env::remove_var("SHELL");

        // When no shell is set in config and no SHELL env var, should default to bash
        assert_eq!(config_no_shell.shell(), "bash");

        // Config with shell set should use that value regardless of SHELL env var
        env::set_var("SHELL", "/usr/bin/zsh");
        assert_eq!(config.shell(), "bash");
        env::remove_var("SHELL");

        // Test auth token overrides
        let model_config = ModelConfig {
            provider: "openai".to_string(),
            model: None,
            auth_token: "config-token".to_string(),
        };

        env::set_var("HAI_OPENAI_TOKEN", "env-token");
        assert_eq!(
            config.get_provider_auth_token("openai", &model_config),
            "env-token"
        );
        env::remove_var("HAI_OPENAI_TOKEN");
        assert_eq!(
            config.get_provider_auth_token("openai", &model_config),
            "config-token"
        );
    }

    #[test]
    fn test_load_config() {
        // Set environment variable to skip interactive setup
        env::set_var("HAI_SKIP_SETUP", "1");

        // This is a basic test that just ensures the function doesn't panic
        // In a real test, we would create a temporary config file and test reading from it
        let result = load_config();

        // Clean up environment variable
        env::remove_var("HAI_SKIP_SETUP");

        assert!(result.is_ok());
    }

    #[test]
    fn test_shell_detection() {
        // Test explicit shell in config
        let config = Config {
            default_model: None,
            temperature: None,
            shell: Some("fish".to_string()),
            history_size: None,
            system_prompt: None,
            max_tokens: None,
            models: None,
        };
        assert_eq!(config.shell(), "fish");

        // Test SHELL environment variable
        let config = Config {
            default_model: None,
            temperature: None,
            shell: None,
            history_size: None,
            system_prompt: None,
            max_tokens: None,
            models: None,
        };
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(config.shell(), "fish");
        env::remove_var("SHELL");
    }
}
