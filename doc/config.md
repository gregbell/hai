% HAI-CONFIG(5) hai 0.1.0

# NAME

hai-config - configuration file for the hai command

# SYNOPSIS

_~/.config/hai/config.toml_

# DESCRIPTION

The **hai** configuration file uses TOML format and defines settings for the
**hai**(1) command, including API credentials, default behaviors, and
customization options.

If the configuration file doesn't exist when **hai** is first run, a default
configuration will be created. You can modify this file to customize the
behavior of **hai**.

# CONFIGURATION OPTIONS

## General Settings

**default-model** : The AI model to use when none is specified. This should
correspond to a model defined in the [models] section. Default: "gpt-4o-mini".

**temperature** : Controls the randomness in AI responses (0.0 to 1.0). Lower
values make responses more deterministic, higher values make responses more
creative. Default: 0.3.

**shell** : The shell to use for executing commands. Supported shells include
bash, zsh, fish, powershell, and pwsh. Defaults to your $SHELL environment
variable on Unix-like systems and PowerShell on Windows.

**history-size** : Maximum number of past commands to keep in history. Default: 50.

**system-prompt** : Specifies the system prompt for the AI model. The default
prompt contains instructions for generating shell commands compatible with the
user's environment. Modifying this value is not recommended for most users and
should be done with caution.

**max-tokens** : Maximum number of tokens in the AI's response. Default: 100.

## Model Settings

Each model configuration under the [models] section requires the following
fields:

**provider** : The AI provider for this model. Valid values are "openai" and
"anthropic".

**model** : The model identifier used by the provider. For OpenAI, examples
include "gpt-4o" and "gpt-3.5-turbo". For Anthropic, examples include
"claude-3-7-sonnet-20250219".

**auth-token** : The API key or authentication token for the specified provider.
This is required to make API calls.

# EXAMPLES

A minimal configuration file:

```toml
default-model = "gpt-4o-mini"

[models.gpt-4o-mini]
provider = "openai"
model = "gpt-4o-mini"
auth-token = "sk-your-openai-api-key"
```

A configuration with multiple models:

```toml
default-model = "gpt-4o-mini"
temperature = 0.5
shell = "zsh"  # Can be "bash", "zsh", "fish", "powershell", or "pwsh"
history-size = 100
system-prompt = "You are a helpful AI that converts natural language to shell commands. Respond with ONLY the shell command, no explanations or markdown formatting. Make sure commands are compatible with the user's environment."
max-tokens = 150

[models.gpt-4o-mini]
provider = "openai"
model = "gpt-4o-mini"
auth-token = "sk-your-openai-api-key"

[models.gpt-3]
provider = "openai"
model = "gpt-3.5-turbo"
auth-token = "sk-your-openai-api-key"

[models.claude-3]
provider = "anthropic"
model = "claude-3-7-sonnet-20250219"
auth-token = "sk-your-anthropic-api-key"
```

# FILES

_~/.config/hai/config.toml_ : User configuration file

# SEE ALSO

**hai**(1)
