use anyhow::Result;
use std::process;

pub fn handle_error(err: anyhow::Error) -> ! {
    eprintln!("Error: {}", err);

    // Check for specific error types and provide more helpful messages
    let err_string = err.to_string();

    if err_string.contains("No models configured")
        || err_string.contains("Model") && err_string.contains("not found in config")
    {
        eprintln!("\nPlease check your configuration file at ~/.config/hai/config.toml");
        eprintln!(
            "Make sure you have configured at least one model and that it has a valid API key."
        );
        eprintln!("You can copy the example configuration from config.toml.example.");
    } else if err_string.contains("Failed to send request to API")
        || err_string.contains("Failed to parse API response")
    {
        eprintln!("\nThere was an issue communicating with the AI service.");
        eprintln!("Please check your internet connection and API key.");
    } else if err_string.contains("Failed to execute command") {
        eprintln!("\nThe command could not be executed.");
        eprintln!("Please check that the required programs are installed.");
    }

    process::exit(1);
}

pub fn run_with_error_handling<F>(f: F) -> !
where
    F: FnOnce() -> Result<()>,
{
    match f() {
        Ok(_) => process::exit(0),
        Err(err) => handle_error(err),
    }
}
