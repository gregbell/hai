# HAI Documentation

This directory contains the documentation for the `hai` command-line tool in Markdown format.

## Files

- `manual.md`: The main manual page for the `hai` command (corresponds to hai(1))
- `config.md`: Documentation for the configuration file format (corresponds to hai-config(5))
- `Makefile`: Makefile for building man pages from Markdown files

## Building Man Pages

To build the man pages from the Markdown files, run:

```bash
# From the doc directory
make

# Or from the project root
make doc
```

This will create man pages in the `man/man1` and `man/man5` directories. Note that these generated man pages are not stored in the repository and are added to `.gitignore`.

## Makefile Targets

The documentation Makefile provides several targets:

- `make` or `make all`: Build all man pages
- `make clean`: Remove generated man pages
- `make install`: Install man pages to system directories (requires root)
- `make uninstall`: Remove man pages from system directories (requires root)
- `make help`: Show available targets

## Requirements

- [pandoc](https://pandoc.org/) is required to convert Markdown to man pages
- Standard Unix tools (`make`, `install`, etc.)

## Viewing Man Pages

After building the man pages, you can view them with:

```bash
man -l ../man/man1/hai.1
man -l ../man/man5/hai-config.5
```

## Installation

When the package is installed, the man pages will be installed to standard man page directories:

- System-wide: `/usr/share/man/man1/hai.1` and `/usr/share/man/man5/hai-config.5`
- User-specific (XDG standard): `~/.local/share/man/man1/hai.1` and `~/.local/share/man/man5/hai-config.5`

The installer will choose the appropriate location based on write permissions. After installation, you can view them with:

```bash
man hai
man hai-config
```

If the man pages were installed to a non-standard location, you may need to add it to your `MANPATH`:

```bash
export MANPATH="$MANPATH:$HOME/.local/share/man"
```
