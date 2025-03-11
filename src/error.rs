use std::process;
use thiserror::Error;

/// Custom error types for the hai application
#[derive(Error, Debug)]
pub enum HaiError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API communication error: {0}")]
    ApiCommunication(String),

    #[error("Command execution error: {0}")]
    CommandExecution(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// Helper functions to create errors
impl HaiError {
    pub fn config<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::Config(msg.into()))
    }

    pub fn io<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::Io(msg.into()))
    }

    pub fn serialization<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::Serialization(msg.into()))
    }

    // These functions are defined but not currently used in the code.
    // They're provided for completeness and future use.
    #[allow(dead_code)]
    pub fn api<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::ApiCommunication(msg.into()))
    }

    #[allow(dead_code)]
    pub fn command<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::CommandExecution(msg.into()))
    }

    #[allow(dead_code)]
    pub fn other<S: Into<String>>(msg: S) -> anyhow::Error {
        anyhow::Error::new(HaiError::Other(msg.into()))
    }
}

/// Converts anyhow::Error to HaiError by examining the error chain
impl From<anyhow::Error> for HaiError {
    fn from(err: anyhow::Error) -> Self {
        // Try to downcast to our custom error type first
        if let Some(hai_err) = err.downcast_ref::<HaiError>() {
            return match hai_err {
                HaiError::Config(s) => HaiError::Config(s.clone()),
                HaiError::ApiCommunication(s) => HaiError::ApiCommunication(s.clone()),
                HaiError::CommandExecution(s) => HaiError::CommandExecution(s.clone()),
                HaiError::Io(s) => HaiError::Io(s.clone()),
                HaiError::Serialization(s) => HaiError::Serialization(s.clone()),
                HaiError::Other(s) => HaiError::Other(s.clone()),
            };
        }

        // Check for known error patterns in the message
        let err_string = err.to_string();

        if err_string.contains("No models configured")
            || (err_string.contains("Model") && err_string.contains("not found in config"))
        {
            return HaiError::Config(err_string);
        } else if err_string.contains("Failed to send request to API")
            || err_string.contains("Failed to parse API response")
        {
            return HaiError::ApiCommunication(err_string);
        } else if err_string.contains("Failed to execute command") {
            return HaiError::CommandExecution(err_string);
        } else if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return HaiError::Io(format!("{}: {}", io_err.kind(), io_err));
        } else if let Some(serde_err) = err.downcast_ref::<serde_json::Error>() {
            return HaiError::Serialization(serde_err.to_string());
        }

        // Default case
        HaiError::Other(err.to_string())
    }
}

/// Handle errors by providing appropriate user feedback and terminating the process
pub fn handle_error(err: anyhow::Error) -> ! {
    let hai_err: HaiError = err.into();

    eprintln!("Error: {}", hai_err);

    // Provide additional helpful context based on error type
    match &hai_err {
        HaiError::Config(_) => {
            eprintln!("\nPlease check your configuration file at ~/.config/hai/config.toml");
            eprintln!(
                "Make sure you have configured at least one model and that it has a valid API key."
            );
            eprintln!("You can copy the example configuration from config.toml.example.");
        }
        HaiError::ApiCommunication(_) => {
            eprintln!("\nThere was an issue communicating with the AI service.");
            eprintln!("Please check your internet connection and API key.");
        }
        HaiError::CommandExecution(_) => {
            eprintln!("\nThe command could not be executed.");
            eprintln!("Please check that the required programs are installed.");
        }
        HaiError::Io(_) => {
            eprintln!("\nThere was an issue with file or network I/O.");
            eprintln!("Please check file permissions and connectivity.");
        }
        HaiError::Serialization(_) => {
            eprintln!("\nThere was an issue parsing or generating data.");
            eprintln!("This might indicate a corrupted configuration or history file.");
        }
        HaiError::Other(_) => {
            eprintln!("\nAn unexpected error occurred.");
            eprintln!("If this persists, please report it as a bug.");
        }
    }

    process::exit(1);
}

pub fn run_with_error_handling<F>(f: F) -> !
where
    F: FnOnce() -> anyhow::Result<()>,
{
    match f() {
        Ok(_) => process::exit(0),
        Err(err) => handle_error(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::io;

    #[test]
    fn test_error_helper_functions() {
        // Test config error
        let err = HaiError::config("config error");
        assert!(err.to_string().contains("config error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::Config(_)));

        // Test io error
        let err = HaiError::io("io error");
        assert!(err.to_string().contains("io error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::Io(_)));

        // Test serialization error
        let err = HaiError::serialization("serialization error");
        assert!(err.to_string().contains("serialization error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::Serialization(_)));

        // Test api error
        let err = HaiError::api("api error");
        assert!(err.to_string().contains("api error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::ApiCommunication(_)));

        // Test command error
        let err = HaiError::command("command error");
        assert!(err.to_string().contains("command error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::CommandExecution(_)));

        // Test other error
        let err = HaiError::other("other error");
        assert!(err.to_string().contains("other error"));
        let downcast_err = err.downcast_ref::<HaiError>().unwrap();
        assert!(matches!(downcast_err, HaiError::Other(_)));
    }

    #[test]
    fn test_from_anyhow_error_direct_conversion() {
        // Test conversion from anyhow::Error containing HaiError directly
        let original = HaiError::Config("test config error".to_string());
        let anyhow_err = anyhow::Error::new(original);
        let converted: HaiError = anyhow_err.into();

        assert!(matches!(converted, HaiError::Config(s) if s == "test config error"));
    }

    #[test]
    fn test_from_anyhow_error_pattern_matching() {
        // Test conversion based on error message patterns

        // Config error pattern
        let anyhow_err = anyhow!("No models configured");
        let hai_err: HaiError = anyhow_err.into();
        assert!(matches!(hai_err, HaiError::Config(_)));

        let anyhow_err = anyhow!("Model XYZ not found in config");
        let hai_err: HaiError = anyhow_err.into();
        assert!(matches!(hai_err, HaiError::Config(_)));

        // API error pattern
        let anyhow_err = anyhow!("Failed to send request to API");
        let hai_err: HaiError = anyhow_err.into();
        assert!(matches!(hai_err, HaiError::ApiCommunication(_)));

        let anyhow_err = anyhow!("Failed to parse API response");
        let hai_err: HaiError = anyhow_err.into();
        assert!(matches!(hai_err, HaiError::ApiCommunication(_)));

        // Command error pattern
        let anyhow_err = anyhow!("Failed to execute command");
        let hai_err: HaiError = anyhow_err.into();
        assert!(matches!(hai_err, HaiError::CommandExecution(_)));
    }

    #[test]
    fn test_from_anyhow_error_io_error() {
        // Test conversion from IO error
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let anyhow_err = anyhow::Error::from(io_err);
        let hai_err: HaiError = anyhow_err.into();

        assert!(matches!(hai_err, HaiError::Io(_)));
        assert!(hai_err.to_string().contains("not found"));
    }

    #[test]
    fn test_from_anyhow_error_serde_error() {
        // Test conversion from serde_json::Error
        let json = r#"{"invalid": json"#;
        let serde_err = serde_json::from_str::<serde_json::Value>(json).unwrap_err();
        let anyhow_err = anyhow::Error::from(serde_err);
        let hai_err: HaiError = anyhow_err.into();

        assert!(matches!(hai_err, HaiError::Serialization(_)));
        assert!(hai_err.to_string().contains("expected")); // Error message usually contains "expected"
    }

    #[test]
    fn test_from_anyhow_error_other() {
        // Test fallback to Other for unknown error types
        let anyhow_err = anyhow!("Some random error");
        let hai_err: HaiError = anyhow_err.into();

        assert!(matches!(hai_err, HaiError::Other(_)));
        assert!(hai_err.to_string().contains("Some random error"));
    }

    #[test]
    fn test_from_anyhow_error_with_context() {
        // Test conversion with context
        // The implementation might not preserve all context in the same way
        // So we'll just test that an error chain with context gets converted to HaiError
        let base_err = anyhow!("Base error");
        let with_context = base_err.context("Additional context");
        let hai_err: HaiError = with_context.into();

        // Just verify we get the expected Other error type
        assert!(matches!(hai_err, HaiError::Other(_)));
        // And that the message contains something meaningful
        assert!(!hai_err.to_string().is_empty());
    }
}
