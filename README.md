# zed-codeql

CodeQL language support for [Zed](https://zed.dev).

## Features

- Syntax highlighting for `.ql`, `.qll`, `.qlref`, and `.dbscheme` files
- Language server integration with CodeQL CLI
- Code completion
- Go to definition
- Hover documentation
- Error diagnostics

## Requirements

CodeQL CLI must be installed and available in your PATH.

### Installation via Homebrew
```bash
brew install codeql
```

### Manual Installation
Download from [GitHub CodeQL releases](https://github.com/github/codeql-action/releases).

## Installation

Install from Zed's extension gallery or build from source.

## Language Server

The extension automatically starts the CodeQL language server when you open a CodeQL file.

The extension searches for CodeQL CLI in your system PATH.

## Credit

Inspired by <https://github.com/pwntester/codeql.nvim>

## License

MIT Copyright (c) 2025 blacktop
