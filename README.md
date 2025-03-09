# hai

Meet **hai**, a tiny CLI tool that turns natural language into Bash or Zsh
commands. You describe what you want to do, hai suggests a command, and asks if
you'd like to run it. A simple, unix-y tool that does one thing /really/ well.

![Hai creating a compressed tarball](assets/tarball.gif)

```console
$ hai "use pandoc to convert all the markdown files in this directory to an ebook"
Command: pandoc -f markdown -t epub -o book.epub *.md
✔ Looks good?
```

Just describe what you want:

```console
hai "copy all txt files to the backup directory"
```

Or pipe stuff into it `hai`:

```basconsole
cat some-file | hai
```

Some more examples:

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

## Documentation

Full documentation is available in the man pages:

- [hai(1)](doc/manual.md) - Main command documentation
- [hai-config(5)](doc/config.md) - Configuration file documentation

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

## Contributing

We would love your help with Hai! If you want to discuss Hai, The best place
is the Github discussions: https://github.com/gregbell/hai/discussions.

If you run into a bug or want to contribute with code, open an issue or PR. We'd
love to see them.

hai is written in Rust. To build from source:

```bash
git clone https://github.com/gregbell/hai.git
cd hai

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

## License

This project is Copyright 2025 Greg Bell and licensed under the GPL-3.0. See [LICENSE](LICENSE).
