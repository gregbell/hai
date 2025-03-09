# Makefile for generating GIFs from VHS tape files
# Requires VHS: https://github.com/charmbracelet/vhs

# Find all .tape files in the current directory
TAPE_FILES := $(wildcard *.tape)
# Generate corresponding .gif filenames
GIF_FILES := $(TAPE_FILES:.tape=.gif)

# Default target: build all GIFs
all: check-vhs $(GIF_FILES)

# Check if VHS is installed
check-vhs:
	@which vhs > /dev/null || (echo "Error: VHS is not installed. Please install it from https://github.com/charmbracelet/vhs" && exit 1)

# Rule to convert a .tape file to a .gif file
%.gif: %.tape
	@echo "Generating $@ from $<..."
	vhs $<

# Clean generated GIFs
clean:
	rm -f $(GIF_FILES)

# List all available tape files
list:
	@echo "Available tape files:"
	@for file in $(TAPE_FILES); do \
		echo "  $$file"; \
	done

# Help target
help:
	@echo "Available targets:"
	@echo "  all (default) - Generate all GIFs from tape files"
	@echo "  clean         - Remove all generated GIFs"
	@echo "  list          - List all available tape files"
	@echo "  help          - Show this help message"
	@echo
	@echo "Individual GIFs can be built by specifying the filename:"
	@echo "  make tarball.gif"

.PHONY: all check-vhs clean list help 