% HAI(1) hai 0.2.0

# NAME

hai - a tiny tool to convert natural language to shell commands

# SYNOPSIS

**hai** [*OPTIONS*] [*PROMPT*]

# DESCRIPTION

**hai** is a command-line utility that converts natural language prompts into
shell commands using the AI providers of your choice. It provides an intuitive
interface for users who know what they want to accomplish but aren't sure about
the exact command syntax.

Currently, **hai** supports OpenAI and Anthropic, and offers features such as
command history, interactive approval of suggested commands, and customizable
system prompts.

When given a natural language prompt, **hai** sends it to the configured AI
provider along with system information about the user's environment to ensure
the generated commands are compatible with the user's system. It supports
multiple shells including Bash, Zsh, Fish, and PowerShell.

# OPTIONS

**-y**, **--yes**
: Skip the prompt and automatically execute the suggested command

**-n**, **--no-execute**
: Show the command, but don't run it

**-m**, **--model** _MODEL_
: Select the model to use (gpt-4, claude-3, etc.)

**-H**, **--history**
: Show command history

**-v**, **--version**
: Show the version information

**-h**, **--help**
: Display help information

# CONFIGURATION

**hai** can be configured through a TOML configuration file located at
`~/.config/hai/config.toml`. The configuration file allows you to set default
behaviors, API credentials, and customize the system prompt.

See **hai-config**(5) for detailed information about the configuration file
format.

# FILES

_~/.config/hai/config.toml_ : User configuration file

_~/.local/share/hai/history.json_ : Command history file

# ENVIRONMENT

**HAI_DEFAULT_MODEL**
: Override the default model to use

**HAI_OPENAI_TOKEN**
: Set the OpenAI API token

**HAI_ANTHROPIC_TOKEN**
: Set the Anthropic API token

# EXAMPLES

Convert a natural language request into a shell command:

    $ hai "find all png files in the current directory"
    find . -name "*.png"

Display command history:

    $ hai --history
    Date       Prompt                         Command                        Model      Executed
    --------------------------------------------------------------------------------------
    2025-03-07 find files modified today      find . -type f -mtime -1       gpt-4      Yes
    2025-03-06 check disk space               df -h                          claude-3   No

Use a specific model:

    $ hai --model claude-3 "show top processes by memory usage"
    ps aux --sort=-%mem | head -n 10

Automatically execute a command:

    $ hai -y "count words in README.md"
    wc -w README.md
          746 README.md

# EXIT STATUS

**0** : Success

**1** : General error

**2** : Configuration error

**3** : Network or API error

# BUGS

Report bugs to: <https://github.com/gregbell/hai/issues>

# AUTHOR

Written by Greg Bell <code@gregbell.ca>

# COPYRIGHT

Copyright © 2025 Greg Bell. License GPLv3+: GNU GPL version 3 or later
<https://gnu.org/licenses/gpl.html>. This is free software: you are free to
change and redistribute it. There is NO WARRANTY, to the extent permitted by
law.

# SEE ALSO

**hai-config**(5)
