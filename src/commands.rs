use std::io::Write;
use std::process::{Command as StdCommand, Stdio};
use zed_extension_api::{self as zed, Command, Result, Worktree};

pub struct QueryExecutor {
    codeql_path: String,
}

impl QueryExecutor {
    pub fn new(codeql_path: String) -> Self {
        Self { codeql_path }
    }

    pub fn run_query(&self, query_path: &str, database_path: &str) -> Result<String> {
        let output = StdCommand::new(&self.codeql_path)
            .args(&[
                "query",
                "run",
                "--database",
                database_path,
                query_path,
                "--format=sarif-latest",
            ])
            .output()
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Query execution failed: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn quick_eval(
        &self,
        query_path: &str,
        database_path: &str,
        predicate: &str,
    ) -> Result<String> {
        let output = StdCommand::new(&self.codeql_path)
            .args(&[
                "query",
                "eval",
                "--database",
                database_path,
                query_path,
                "--predicate",
                predicate,
            ])
            .output()
            .map_err(|e| format!("Failed to evaluate predicate: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Quick evaluation failed: {}", error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn list_databases(&self, workspace_path: &str) -> Result<Vec<String>> {
        let output = StdCommand::new(&self.codeql_path)
            .args(&["database", "list", "--format=json"])
            .current_dir(workspace_path)
            .output()
            .map_err(|e| format!("Failed to list databases: {}", e))?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        // Parse JSON output to extract database names
        // For now, return a simple list
        Ok(vec![])
    }

    pub fn create_database(
        &self,
        source_path: &str,
        database_path: &str,
        language: &str,
    ) -> Result<()> {
        let output = StdCommand::new(&self.codeql_path)
            .args(&[
                "database",
                "create",
                database_path,
                "--language",
                language,
                "--source-root",
                source_path,
            ])
            .output()
            .map_err(|e| format!("Failed to create database: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Database creation failed: {}", error));
        }

        Ok(())
    }

    pub fn upgrade_database(&self, database_path: &str) -> Result<()> {
        let output = StdCommand::new(&self.codeql_path)
            .args(&["database", "upgrade", database_path])
            .output()
            .map_err(|e| format!("Failed to upgrade database: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Database upgrade failed: {}", error));
        }

        Ok(())
    }
}
