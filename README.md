# zed-codeql

CodeQL language support for [Zed](https://zed.dev).

## Features

- Syntax highlighting for `.ql`, `.qll`, `.qlref`, and `.dbscheme` files
- Language server integration with CodeQL CLI
- Code completion with CodeQL-specific formatting
- Go to definition, hover documentation, and diagnostics
- Code folding, auto-indentation, and bracket matching
- Outline view for code navigation
- Syntax injection for SQL, JavaScript, HTML, JSON, and YAML in strings
- Support for QL pack configuration files (`qlpack.yml`)

## Installation

### Prerequisites

Install CodeQL CLI:
```bash
# macOS with Homebrew
brew install --cask codeql

# Or download from https://github.com/github/codeql-cli-binaries
```

### Installing the Extension

Install via Zed's extension manager.

## Configuration

The extension looks for CodeQL CLI in:
- `/opt/homebrew/bin/codeql` (Homebrew)
- System PATH

## Language Server Settings

The extension configures the CodeQL language server with:
- Format on save enabled
- 2GB memory limit for running queries
- 300 second timeout for queries
- Automatic database downloads for CodeQL workspaces

## License

MIT Copyright (c) 2025 blacktop
