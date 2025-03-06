#[cfg(test)]
mod tests {
    use std::process::Command;
    
    #[test]
    fn test_help_flag() {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        assert!(stdout.contains("Usage:"));
        assert!(stdout.contains("Options:"));
        assert!(stdout.contains("--yes"));
        assert!(stdout.contains("--no-execute"));
        assert!(stdout.contains("--model"));
    }
    
    #[test]
    fn test_version_flag() {
        let output = Command::new("cargo")
            .args(["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        assert!(stdout.contains("hai"));
        assert!(stdout.contains("0.1.0"));
    }
} 