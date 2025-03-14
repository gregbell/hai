# Hai Installer Scripts

This directory contains scripts that are used during the installation process of Hai.

## Man Page Installation

Hai includes man pages for easy reference:

- `hai(1)` - Main command documentation
- `hai-config(5)` - Configuration file documentation

### How Man Pages Are Installed

1. When you install Hai using the shell installer (on macOS/Linux) or PowerShell installer (on Windows), the post-install scripts in this directory are executed.
2. The scripts copy the man pages to the appropriate location based on your system:
   - On Linux: Uses XDG standard locations (`$XDG_DATA_HOME/man` or `$HOME/.local/share/man`) if system directories aren't writable
   - On macOS: Uses `/usr/local/share/man` if writable, otherwise falls back to XDG paths
   - On Windows: Uses appropriate Windows-specific paths
3. For Homebrew installations, the man pages are automatically installed to the Homebrew man directory.

### Accessing Man Pages

On macOS/Linux:

```bash
# View the main command documentation
man hai

# View the configuration file documentation
man hai-config
```

On Windows, you have several options:

1. Use Git Bash, which includes the `man` command
2. Use Windows Subsystem for Linux (WSL)
3. Open the man page files directly with a text editor
4. Install a third-party man page viewer

### Troubleshooting

If the man pages aren't accessible after installation on Linux/macOS and they were installed to a non-standard location, you may need to add that location to your `MANPATH`:

```bash
# If installed to ~/.local/share/man
export MANPATH="$MANPATH:$HOME/.local/share/man"
```

Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.) to make it permanent.
