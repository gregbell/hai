#!/bin/bash
set -e

# Check if a version is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

VERSION=$1

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Build the release
cargo build --release

# Create a release directory
RELEASE_DIR="release/hai-$VERSION"
mkdir -p "$RELEASE_DIR"

# Copy the binary and other files
cp target/release/hai "$RELEASE_DIR/"
cp README.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/" 2>/dev/null || echo "LICENSE file not found, skipping"

# Create a tarball
cd release
tar -czf "hai-$VERSION.tar.gz" "hai-$VERSION"
cd ..

echo "Release created: release/hai-$VERSION.tar.gz"
echo "Don't forget to commit the version change and create a git tag:"
echo "git commit -am \"Bump version to $VERSION\""
echo "git tag v$VERSION"
echo "git push && git push --tags" 