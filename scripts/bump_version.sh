#!/usr/bin/env bash
set -euo pipefail

# Script to bump version numbers across the hai project
# Usage: ./scripts/bump_version.sh <new_version>

# Check if a version was provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (simple check for x.y.z format)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format x.y.z (e.g., 0.2.0)"
    exit 1
fi

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep -m 1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"
echo "New version: $NEW_VERSION"

# Confirm with the user
read -p "Are you sure you want to update from $CURRENT_VERSION to $NEW_VERSION? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

# Update version in Cargo.toml
echo "Updating version in Cargo.toml..."
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update version in doc/manual.md
echo "Updating version in doc/manual.md..."
sed -i "s/% HAI(1) hai $CURRENT_VERSION/% HAI(1) hai $NEW_VERSION/" doc/manual.md

# Update version in doc/config.md
echo "Updating version in doc/config.md..."
sed -i "s/% HAI-CONFIG(5) hai $CURRENT_VERSION/% HAI-CONFIG(5) hai $NEW_VERSION/" doc/config.md

# Update version in tests/cli_tests.rs
echo "Updating version in tests/cli_tests.rs..."
sed -i "s/assert!(stdout.contains(\"$CURRENT_VERSION\"));/assert!(stdout.contains(\"$NEW_VERSION\"));/" tests/cli_tests.rs

# Update version in CHANGELOG.md
# This is more complex as we need to add a new entry
echo "Updating CHANGELOG.md..."
TODAY=$(date +%Y-%m-%d)
sed -i "/^## v$CURRENT_VERSION/i ## v$NEW_VERSION ($TODAY)\n\n* \n" CHANGELOG.md

# Update version in README.md
echo "Updating version in README.md..."
sed -i "s/make release VERSION=$CURRENT_VERSION/make release VERSION=$NEW_VERSION/" README.md
sed -i "s/git tag v$CURRENT_VERSION/git tag v$NEW_VERSION/" README.md
sed -i "s/git push origin v$CURRENT_VERSION/git push origin v$NEW_VERSION/" README.md

echo "Version updated successfully to $NEW_VERSION"
echo "Please review the changes and update the CHANGELOG.md with the new features/fixes."
echo "Then commit the changes with: git commit -am \"Bump version to $NEW_VERSION\""
echo "After committing, tag the release with: git tag v$NEW_VERSION && git push origin v$NEW_VERSION" 