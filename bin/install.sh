#!/bin/bash
set -e

echo "Installing hai..."

# Build the binary
cargo build --release

# Install the binary
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    cp target/release/hai "$INSTALL_DIR/hai"
    echo "Installed hai to $INSTALL_DIR/hai"
else
    echo "Cannot write to $INSTALL_DIR. You may need to run with sudo."
    echo "To install manually, copy target/release/hai to a directory in your PATH"
fi

echo "Installation complete!"
echo "Run 'hai' to get started and set up your configuration" 