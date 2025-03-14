# Main Makefile for hai

.PHONY: all build test doc clean install uninstall help release gifs bump-version

# Default target
all: build doc

# Build the application
build:
	cargo build --release

# Run tests
test:
	cargo test

# Build documentation
doc:
	$(MAKE) -C doc

# Generate GIFs in ./assets
gifs:
	$(MAKE) -C assets

# Clean build artifacts
clean:
	cargo clean
	$(MAKE) -C doc clean
	$(MAKE) -C assets clean
	rm -rf release

# Install the application and documentation
install: build doc
	# System-wide installation (requires root)
	install -d $(DESTDIR)/usr/bin
	install -m 755 target/release/hai $(DESTDIR)/usr/bin/
	$(MAKE) -C doc install
	@echo "Installation complete!"
	@echo "Run 'hai' to get started and set up your configuration"

# Local installation to /usr/local/bin
local-install: build doc
	@if [ -w "/usr/local/bin" ]; then \
		install -m 755 target/release/hai /usr/local/bin/hai; \
		echo "Installed hai to /usr/local/bin/hai"; \
	else \
		echo "Cannot write to /usr/local/bin. You may need to run with sudo."; \
		echo "To install manually, copy target/release/hai to a directory in your PATH"; \
	fi
	@echo "Installation complete!"
	@echo "Run 'hai' to get started and set up your configuration"

# Uninstall the application and documentation
uninstall:
	rm -f $(DESTDIR)/usr/bin/hai
	$(MAKE) -C doc uninstall

# Local uninstall from /usr/local/bin
local-uninstall:
	rm -f /usr/local/bin/hai

# Create a Debian package
deb: build doc
	# Ensure man pages are built before packaging
	mkdir -p man/man1 man/man5
	$(MAKE) -C doc
	# Build the Debian package with all documentation
	cargo deb

# Create a release tarball and Debian package
# Usage: make release VERSION=0.1.0
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Usage: make release VERSION=x.y.z"; \
		exit 1; \
	fi
	@echo "Creating release for version $(VERSION)"
	# Update version in Cargo.toml
	sed -i "s/^version = \".*\"/version = \"$(VERSION)\"/" Cargo.toml
	# Build the release
	cargo build --release
	# Create a release directory
	mkdir -p release/hai-$(VERSION)
	# Copy the binary and other files
	cp target/release/hai release/hai-$(VERSION)/
	cp README.md release/hai-$(VERSION)/
	cp LICENSE release/hai-$(VERSION)/ 2>/dev/null || echo "LICENSE file not found, skipping"
	# Build documentation
	$(MAKE) -C doc
	mkdir -p release/hai-$(VERSION)/man/man1 release/hai-$(VERSION)/man/man5
	cp man/man1/hai.1 release/hai-$(VERSION)/man/man1/
	cp man/man5/hai-config.5 release/hai-$(VERSION)/man/man5/
	cp -r doc release/hai-$(VERSION)/
	# Create a tarball
	cd release && tar -czf hai-$(VERSION).tar.gz hai-$(VERSION)
	# Build Debian package
	$(MAKE) deb
	# Copy the Debian package to the release directory
	mkdir -p release/hai-$(VERSION)/deb
	cp target/debian/hai_$(VERSION)*.deb release/hai-$(VERSION)/deb/
	cp target/debian/hai_$(VERSION)*.deb release/
	@echo "Release created: release/hai-$(VERSION).tar.gz"
	@echo "Debian package: release/hai_$(VERSION)*.deb"

# Bump version across all project files
# Usage: make bump-version VERSION=0.2.0
bump-version:
	@if [ -z "$(VERSION)" ]; then \
		echo "Usage: make bump-version VERSION=x.y.z"; \
		exit 1; \
	fi
	@echo "Bumping version to $(VERSION)"
	./scripts/bump_version.sh $(VERSION)

# Help target
help:
	@echo "Available targets:"
	@echo "  all (default)    - Build the application and documentation"
	@echo "  build            - Build the application"
	@echo "  test             - Run tests"
	@echo "  doc              - Build documentation"
	@echo "  gifs             - Generate GIFs in ./assets"
	@echo "  clean            - Remove build artifacts"
	@echo "  install          - Install the application and documentation to system directories"
	@echo "  local-install    - Install the application to /usr/local/bin"
	@echo "  uninstall        - Remove the application and documentation from system directories"
	@echo "  local-uninstall  - Remove the application from /usr/local/bin"
	@echo "  deb              - Create a Debian package with documentation"
	@echo "  release          - Create a local release tarball for testing (Usage: make release VERSION=x.y.z)"
	@echo "  bump-version     - Bump version across all project files (Usage: make bump-version VERSION=x.y.z)"
	@echo "  help             - Show this help message"
	@echo
	@echo "Documentation targets (run with make -C doc):"
	@$(MAKE) -C doc help
	@echo
	@echo "GIF targets (run with make -C assets):"
	@$(MAKE) -C assets help 