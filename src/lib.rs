use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, Command, LanguageServerId, Result, Worktree,
};

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
        // This works properly in WASM environment
        if let Some(path) = worktree.which("codeql") {
            // Cache the path for future use
            self.cached_binary_path = Some(path.clone());
            Some(path)
        } else {
            None
        }
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
            concat!(
                "CodeQL CLI not found in PATH.\n",
                "Please install CodeQL CLI:\n",
                "  • Homebrew: brew install codeql\n",
                "  • Download: https://github.com/github/codeql-action/releases\n",
                "Then restart Zed to detect the installation."
            ).to_string()
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
        _server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // CodeQL language server configuration options
        // Using conservative defaults that work on most systems
        let settings = serde_json::json!({
            "codeql": {
                "cli": {
                    "executablePath": self.find_codeql_cli(worktree)
                },
                "runningQueries": {
                    "numberOfThreads": 2,     // Conservative default for compatibility
                    "saveCache": false,
                    "cacheSize": 1024,        // 1GB cache size
                    "timeout": 600,           // 10 minutes for complex queries
                    "memory": 1024,           // 1GB memory limit
                    "debug": false,
                    "customLogDirectory": ""
                },
                "runningTests": {
                    "numberOfThreads": 1,     // Single thread for tests
                    "saveCache": false,
                    "cacheSize": 512,         // 512MB cache for tests
                    "timeout": 300,           // 5 minutes for tests
                    "memory": 512,            // 512MB memory for tests
                    "customLogDirectory": ""
                }
            }
        });

        Ok(Some(settings))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            zed::lsp::CompletionKind::Class | zed::lsp::CompletionKind::Module => {
                let label = completion.label.clone();
                let name_span = CodeLabelSpan::literal(&label, Some("type".to_string()));
                Some(CodeLabel {
                    code: label,
                    spans: vec![name_span],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            zed::lsp::CompletionKind::Method | zed::lsp::CompletionKind::Function => {
                let label = format!("{}()", completion.label);
                let name_span =
                    CodeLabelSpan::literal(&completion.label, Some("function".to_string()));
                let paren_span = CodeLabelSpan::literal("()", None);
                Some(CodeLabel {
                    code: label.clone(),
                    spans: vec![name_span, paren_span],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            zed::lsp::CompletionKind::Keyword => {
                let name_span =
                    CodeLabelSpan::literal(&completion.label, Some("keyword".to_string()));
                Some(CodeLabel {
                    code: completion.label.clone(),
                    spans: vec![name_span],
                    filter_range: (0..completion.label.len()).into(),
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
        let name = symbol.name.clone();

        let (code, highlight) = match symbol.kind {
            zed::lsp::SymbolKind::Class | zed::lsp::SymbolKind::Interface => {
                (format!("class {}", name), Some("type".to_string()))
            }
            zed::lsp::SymbolKind::Method | zed::lsp::SymbolKind::Function => {
                (format!("{}()", name), Some("function".to_string()))
            }
            zed::lsp::SymbolKind::Module | zed::lsp::SymbolKind::Namespace => {
                (format!("module {}", name), Some("module".to_string()))
            }
            _ => (name.clone(), None),
        };

        let name_span = CodeLabelSpan::literal(&name, highlight);
        Some(CodeLabel {
            code: code.clone(),
            spans: vec![name_span],
            filter_range: (0..name.len()).into(),
        })
    }
}

zed::register_extension!(CodeQLExtension);
