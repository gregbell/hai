#!/bin/bash
# Post-installation script for hai
# This script installs man pages to the appropriate location

set -e

# Determine the appropriate man page location based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS standard location
    if [ -w "/usr/local/share/man" ]; then
        MAN_DIR="/usr/local/share/man"
    else
        # Fallback to user's home directory if system directories aren't writable
        MAN_DIR="$HOME/.local/share/man"
        echo "Note: Using $MAN_DIR as system directories aren't writable"
    fi
else
    # Linux standard locations
    if [ -w "/usr/local/share/man" ]; then
        MAN_DIR="/usr/local/share/man"
    elif [ -w "/usr/share/man" ]; then
        MAN_DIR="/usr/share/man"
    else
        # Fallback to XDG standard for user-specific man pages
        MAN_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/man"
        echo "Note: Using $MAN_DIR as system directories aren't writable"
    fi
fi

# Create man directories if they don't exist
MAN1_DIR="$MAN_DIR/man1"
MAN5_DIR="$MAN_DIR/man5"

mkdir -p "$MAN1_DIR" "$MAN5_DIR"

# Copy man pages
if [ -f "man/man1/hai.1" ]; then
    cp "man/man1/hai.1" "$MAN1_DIR/"
    echo "Installed man page: hai(1) to $MAN1_DIR"
fi

if [ -f "man/man5/hai-config.5" ]; then
    cp "man/man5/hai-config.5" "$MAN5_DIR/"
    echo "Installed man page: hai-config(5) to $MAN5_DIR"
fi

echo "Man pages installed to $MAN_DIR"
echo "You can view them with: man hai or man hai-config"

# If we used a non-standard location, provide MANPATH instructions
if [[ "$MAN_DIR" == "$HOME/.local/share/man" || "$MAN_DIR" == "${XDG_DATA_HOME:-$HOME/.local/share}/man" ]]; then
    echo ""
    echo "NOTE: You may need to add $MAN_DIR to your MANPATH"
    echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export MANPATH=\"\$MANPATH:$MAN_DIR\""
fi

exit 0 