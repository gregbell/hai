# hai

Meet **hai**, a tiny CLI tool that turns natural language into Bash or Zsh
commands. You describe what you want to do, hai suggests a command, and asks if
you'd like to run it. A simple, unix-y tool that does one thing /really/ well.

```console
$ hai "use pandoc to convert all the markdown files in this directory to an ebook"
Command: pandoc -f markdown -t epub -o book.epub *.md
✔ Looks good?
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
   # Build and install to /usr/local/bin (may require sudo)
   make local-install

   # Or for a system-wide installation
   sudo make install
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
hai "copy all txt files to the backup directory"
```

Or pipe stuff into it `hai`:

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

## More examples

```console
$ hai "find and delete all log files"
Command: find . -name "*.log" -delete
```

```console
$ hai "list all running docker containers"
Command: docker ps
```

```console
$ hai "find all jpg files and resize them to 800px wide"
Command: find . -name "*.jpg" -print0 | xargs -0 mogrify -resize 800x
```

```console
$ hai "search all markdown files for TODO and save the results to todos.txt"
Command: grep -r "TODO" *.md > todos.txt
```

## Configuration

hai's configuration is managed via a `config.toml` file located at
`~/.config/hai/config.toml`. This file is created automatically when you first
run hai, and you'll be guided through the setup process.

### Configuration Options

```toml
# The default model to use when --model is not specified
# Default: "gpt-4o-mini"
default-model = "gpt-4o-mini"

# Controls the randomness in AI responses (0.0 to 1.0)
# Lower values make responses more deterministic
# Higher values make responses more creative
# Default: 0.3
temperature = 0.5

# The shell to use for executing commands
# Default: "bash"
shell = "bash"

# Maximum number of past commands to keep in history
# Default: 50
history-size = 50

# System prompt used to guide the AI's behavior
# Default: "You are a helpful AI that converts natural language to shell commands. Respond with ONLY the shell command, no explanations or markdown formatting."
system-prompt = "You are a helpful AI that converts natural language to shell commands."

# Maximum number of tokens in the AI's response
# Default: 100
max-tokens = 100

# Model configurations
[models]

# Example OpenAI configuration
[models.gpt-4o-mini]
provider = "openai"   # Required: The AI provider to use ("openai" or "anthropic")
model = "gpt-4o-mini" # Optional: The specific model to use (defaults to key name if not specified)
auth-token = ""       # Required: Your API authentication token

# Example Anthropic configuration
[models.claude-3]
provider = "anthropic"
model = "claude-3-opus-20240229"
auth-token = ""
```

### Model Configuration

Each model in the `[models]` section requires:

- `provider`: The AI provider to use (currently supported: "openai" or
  "anthropic")
- `auth-token`: Your API authentication token for the provider
- `model`: (Optional) The specific model identifier. If not specified, uses the
  configuration key name

The configuration supports multiple models, allowing you to switch between them
using the `--model` flag:

```bash
hai --model claude-3 "list all docker containers"
```

### Environment Variables

You can use environment variables to override configuration values:

- `HAI_DEFAULT_MODEL`: Override the default model to use
- `HAI_OPENAI_TOKEN`: Set the OpenAI API token
- `HAI_ANTHROPIC_TOKEN`: Set the Anthropic API token
- `SHELL`: Override the shell used for executing commands (defaults to "bash" if
  not set)

Environment variables take precedence over values in the config file. This is
useful for:

- Temporarily switching models: `HAI_DEFAULT_MODEL=claude-3 hai "list files"`
- Using different API keys: `HAI_OPENAI_TOKEN=sk-... hai "list files"`
- Running with a different shell: `SHELL=zsh hai "list files"`

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

The history is stored in `~/.config/hai/history.json`. You can configure the
maximum number of commands to keep in history by setting the `history-size`
option in your config file.

## Development

### Building

hai uses a Makefile system for building, testing, and installation:

```bash
# Build the application and documentation
make

# Run tests
make test

# Build only the documentation
make doc

# Clean build artifacts
make clean
```

### Creating a Release

To create a release tarball:

```bash
make release VERSION=0.1.0
```

This will:

1. Update the version in Cargo.toml
2. Build the application
3. Create a release tarball with the binary, documentation, and man pages
4. Output instructions for creating a git tag

### Man Pages

hai includes comprehensive man pages for both the command and its configuration file:

```bash
# After installation
man hai
man hai-config

# During development (after building documentation)
man -l man/man1/hai.1
man -l man/man5/hai-config.5
```

The man pages are maintained in Markdown format in the `doc` directory and converted to man pages using pandoc.

## Contributing

Open an issue or PR if you've got ideas or fixes. We'd love to see them.

hai is written in Rust. To build from source:

```bash
git clone https://github.com/gregbell/hai.git
cd hai
make
```

## License

This project is Copyright 2025 Greg Bell and licensed under the GPL-3.0. See [LICENSE](LICENSE).
