# Makefile for building hai documentation

# Directories
MAN_DIR = ../man
MAN1_DIR = $(MAN_DIR)/man1
MAN5_DIR = $(MAN_DIR)/man5

# Man pages
MAN1_PAGES = $(MAN1_DIR)/hai.1
MAN5_PAGES = $(MAN5_DIR)/hai-config.5

# Source files
MD_SOURCES = manual.md config.md

# Default target
all: man

# Create directories
$(MAN1_DIR) $(MAN5_DIR):
	mkdir -p $@

# Build man pages
man: check-pandoc $(MAN1_DIR) $(MAN5_DIR) $(MAN1_PAGES) $(MAN5_PAGES)

# Check if pandoc is installed
check-pandoc:
	@which pandoc > /dev/null || (echo "Error: pandoc is not installed. Please install it to build man pages." && exit 1)

# Build hai.1 man page
$(MAN1_DIR)/hai.1: manual.md
	@echo "Building hai(1) man page..."
	pandoc --standalone --from markdown --to man $< -o $@

# Build hai-config.5 man page
$(MAN5_DIR)/hai-config.5: config.md
	@echo "Building hai-config(5) man page..."
	pandoc --standalone --from markdown --to man $< -o $@

# Clean generated files
clean:
	rm -rf $(MAN_DIR)

# Install man pages to system directories (requires root)
install: man
	install -d $(DESTDIR)/usr/share/man/man1
	install -d $(DESTDIR)/usr/share/man/man5
	install -m 644 $(MAN1_PAGES) $(DESTDIR)/usr/share/man/man1/
	install -m 644 $(MAN5_PAGES) $(DESTDIR)/usr/share/man/man5/

# Uninstall man pages from system directories (requires root)
uninstall:
	rm -f $(DESTDIR)/usr/share/man/man1/hai.1
	rm -f $(DESTDIR)/usr/share/man/man5/hai-config.5

# Help target
help:
	@echo "Available targets:"
	@echo "  all (default) - Build all man pages"
	@echo "  man           - Build all man pages"
	@echo "  clean         - Remove generated man pages"
	@echo "  install       - Install man pages to system directories"
	@echo "  uninstall     - Remove man pages from system directories"
	@echo "  help          - Show this help message"

.PHONY: all man clean install uninstall help check-pandoc 