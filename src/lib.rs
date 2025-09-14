use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree, CodeLabel, CodeLabelSpan};

mod commands;

struct CodeQLExtension {
    cached_binary_path: Option<String>,
}

impl CodeQLExtension {
    fn find_codeql_cli(&mut self, worktree: &Worktree) -> Option<String> {
        // Return cached path if available
        if let Some(path) = &self.cached_binary_path {
            return Some(path.clone());
        }

        // Use Zed's which function to find codeql in PATH
        if let Some(path) = worktree.which("codeql") {
            self.cached_binary_path = Some(path.clone());
            return Some(path);
        }

        // Try common installation paths using Zed's which
        let possible_commands = vec![
            "/opt/homebrew/bin/codeql",
        ];

        for cmd in possible_commands {
            if let Some(path) = worktree.which(cmd) {
                self.cached_binary_path = Some(path.clone());
                return Some(path);
            }
        }

        None
    }
}

impl zed::Extension for CodeQLExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let codeql_path = self.find_codeql_cli(worktree).ok_or_else(|| {
            "CodeQL CLI not found. Please install CodeQL from https://github.com/github/codeql-cli-binaries and ensure it's in your PATH.".to_string()
        })?;

        Ok(Command {
            command: codeql_path,
            args: vec![
                "execute".to_string(),
                "language-server".to_string(),
                "--check-errors=ON_CHANGE".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Enhanced workspace detection
        let workspace_root = worktree.root_path();

        // Check for various CodeQL workspace indicators
        let has_ql_files = workspace_root.contains(".ql") || workspace_root.contains(".qll");
        let has_qlpack = workspace_root.contains("qlpack.yml") || workspace_root.contains("qlpack.yaml");
        let has_workspace_config = workspace_root.contains("codeql-workspace.yml") ||
                                   workspace_root.contains(".codeqlrc");
        let has_codeql_dir = workspace_root.contains(".codeql");
        let has_database = workspace_root.contains(".db") || workspace_root.contains("codeql-database");

        let is_codeql_workspace = has_ql_files || has_qlpack || has_workspace_config ||
                                  has_codeql_dir || has_database;

        // Provide comprehensive workspace configuration
        Ok(Some(serde_json::json!({
            "codeQL": {
                "cli": {
                    "executablePath": self.cached_binary_path.clone()
                },
                "runningQueries": {
                    "memory": 2048,
                    "debug": false,
                    "maxPaths": 1000,
                    "timeout": 300
                },
                "runningTests": {
                    "numberOfThreads": 1
                }
            },
            "formatting": {
                "enable": true,
                "formatOnSave": true,
                "formatOnType": false
            },
            "telemetry": {
                "enableTelemetry": false
            },
            "database": {
                "autoDownload": is_codeql_workspace
            }
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let label = &completion.label;
        let detail = completion.detail.as_deref();
        let kind = completion.kind?;

        // Create CodeQL-specific completion labels
        match kind {
            zed::lsp::CompletionKind::Class => {
                Some(CodeLabel {
                    code: format!("class {}", label),
                    spans: vec![CodeLabelSpan::code_range(0..label.len() + 6)],
                    filter_range: 6..6 + label.len(),
                })
            }
            zed::lsp::CompletionKind::Method | zed::lsp::CompletionKind::Function => {
                let signature = detail.unwrap_or(label.as_str());
                Some(CodeLabel {
                    code: format!("predicate {}", signature),
                    spans: vec![CodeLabelSpan::code_range(0..signature.len() + 10)],
                    filter_range: 10..10 + label.len(),
                })
            }
            zed::lsp::CompletionKind::Module => {
                Some(CodeLabel {
                    code: format!("module {}", label),
                    spans: vec![CodeLabelSpan::code_range(0..label.len() + 7)],
                    filter_range: 7..7 + label.len(),
                })
            }
            zed::lsp::CompletionKind::Keyword => {
                Some(CodeLabel {
                    code: label.clone(),
                    spans: vec![CodeLabelSpan::code_range(0..label.len())],
                    filter_range: 0..label.len(),
                })
            }
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;

        match symbol.kind {
            zed::lsp::SymbolKind::Class => {
                Some(CodeLabel {
                    code: format!("class {}", name),
                    spans: vec![CodeLabelSpan::code_range(0..name.len() + 6)],
                    filter_range: 6..6 + name.len(),
                })
            }
            zed::lsp::SymbolKind::Function | zed::lsp::SymbolKind::Method => {
                Some(CodeLabel {
                    code: format!("predicate {}", name),
                    spans: vec![CodeLabelSpan::code_range(0..name.len() + 10)],
                    filter_range: 10..10 + name.len(),
                })
            }
            zed::lsp::SymbolKind::Module | zed::lsp::SymbolKind::Namespace => {
                Some(CodeLabel {
                    code: format!("module {}", name),
                    spans: vec![CodeLabelSpan::code_range(0..name.len() + 7)],
                    filter_range: 7..7 + name.len(),
                })
            }
            _ => None,
        }
    }
}

zed::register_extension!(CodeQLExtension);