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
