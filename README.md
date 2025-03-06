# hai

**hai** is a tiny CLI tool that turns natural language into Bash or Zsh commands using AI. You describe what you want to do, hai suggests a command, and asks if you'd like to run it. A simple, unix-y tool that does one thing well.

```bash
hai use pandoc to convert all the markdown files in this directory to an ebook
```

Output:

```
Command: pandoc -i *.md -o book.epub
Looks good? Y/n
```

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/gregbell/hai.git
   cd hai
   ```

2. Build and install:
   ```bash
   cargo build --release
   sudo cp target/release/hai /usr/local/bin/
   ```

3. Run hai for the first time to set up your configuration:
   ```bash
   hai
   ```
   
   This will guide you through setting up your configuration and API keys.

### From Package Managers

Coming soon!

## Usage

Just describe what you want:

```bash
hai copy all txt files to the backup directory
```

Or pipe stuff into it:

```bash
cat some-file | hai
```

## Flags

- `-y`, `--yes`: Skip the prompt and just run the command.
- `-n`, `--no-execute`: Show the command, but don't run it.
- `-m`, `--model`: Select the model to use
- `-H`, `--history`: Show command history
- `-v`, `--version`: Show the version.
- `-h`, `--help`: Show help.

## Command History

hai keeps track of your command history. You can view your command history with:

```bash
hai --history
```

This will display a table of your past commands, including:
- The date the command was run
- The prompt you used
- The command that was generated
- Whether the command was executed

The history is stored in `~/.config/hai/history.json`. You can configure the maximum number of commands to keep in history by setting the `history-size` option in your config file.

## More examples

```bash
hai find and delete all log files
```

```
Command: find . -name "*.log" -delete
Looks good? Y/n
```

```bash
hai list all running docker containers
```

```
Command: docker ps
Looks good? Y/n
```

### Multi-command examples

```bash
hai find all jpg files and resize them to 800px wide
```

```
Command: find . -name "*.jpg" -print0 | xargs -0 mogrify -resize 800x
Looks good? Y/n
```

```bash
hai search all markdown files for TODO and save the results to todos.txt
```

```
Command: grep -r "TODO" *.md > todos.txt
Looks good? Y/n
```

## Configuration

hai's configuration is managed via a `config.toml` file located at `~/.config/hai/config.toml`. This file is created automatically when you first run hai, and you'll be guided through the setup process.

### Example Configuration

```toml
# Default model to use if --model is not specified
default-model = "gpt-4o"

# Global settings
temperature = 0.7  # Controls randomness of responses
confirm-by-default = false  # If true, executes commands without asking
shell = "bash"  # Default shell for command execution
log-file = "~/.config/hai/hai.log"  # Log file for debugging
history-size = 50  # Number of past commands to keep in history
system-prompt = "You are a helpful AI that converts natural language to shell commands."
max-tokens = 100  # Token limit for response length

# Configuration for OpenAI's GPT-4o model
[models.gpt-4o]
api-url = "https://api.openai.com/v1/chat/completions"
auth-token = "your-openai-api-key"

# Configuration for Anthropic's Claude-3 model
[models.claude-3]
api-url = "https://api.anthropic.com/v1/complete"
auth-token = "your-anthropic-api-key"
```

### Selecting a Model at Runtime

You can specify which AI model to use by passing the `--model` flag followed by the model's name when running hai:

```bash
hai --model "claude-3" "list all running docker containers"
```

If the `--model` flag is omitted, hai will use the model specified in the `default-model` setting of your `config.toml`.

## Development

For development purposes, there are some helper scripts in the `bin` directory:

- `bin/release.sh`: Creates a release tarball
- `bin/install.sh`: Installs the application locally

## Contributing

Open an issue or PR if you've got ideas or fixes. We'd love to see them.

hai is written in Rust. To build from source:

```bash
git clone https://github.com/gregbell/hai.git
cd hai
cargo build --release
```

## License

GPL-3.0. See [LICENSE](LICENSE).