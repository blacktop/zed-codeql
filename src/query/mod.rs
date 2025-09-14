// Query execution module - Example implementation demonstrating architecture

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use zed_extension_api as zed;

/// Represents a CodeQL query that can be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Path to the .ql file
    pub path: PathBuf,
    /// Optional query metadata
    pub metadata: Option<QueryMetadata>,
    /// Query timeout in seconds
    pub timeout: u32,
}

/// Query metadata extracted from QL documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetadata {
    pub name: String,
    pub description: Option<String>,
    pub severity: Option<String>,
    pub tags: Vec<String>,
}

/// Results from query execution
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub query_path: PathBuf,
    pub database_path: PathBuf,
    pub sarif: Option<SarifResults>,
    pub raw_output: String,
    pub execution_time: Duration,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Simplified SARIF results structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SarifResults {
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifResult {
    pub rule_id: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifLocation {
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    pub artifact_location: SarifArtifactLocation,
    pub region: Option<SarifRegion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SarifRegion {
    pub start_line: u32,
    pub start_column: Option<u32>,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
}

/// Main query executor that handles running CodeQL queries
pub struct QueryExecutor {
    codeql_path: String,
    active_database: Option<PathBuf>,
    max_paths: u32,
    num_threads: u32,
}

impl QueryExecutor {
    /// Creates a new query executor with the given CodeQL CLI path
    pub fn new(codeql_path: String) -> Self {
        Self {
            codeql_path,
            active_database: None,
            max_paths: 100,
            num_threads: 4,
        }
    }

    /// Sets the active database for query execution
    pub fn set_database(&mut self, database_path: PathBuf) -> Result<(), String> {
        // Validate that the database exists and is valid
        if !database_path.exists() {
            return Err(format!("Database does not exist: {:?}", database_path));
        }

        // Check for database metadata file
        let metadata_path = database_path.join("codeql-database.yml");
        if !metadata_path.exists() {
            return Err(format!("Invalid CodeQL database: missing metadata file"));
        }

        self.active_database = Some(database_path);
        Ok(())
    }

    /// Executes a query against the active database
    pub async fn execute_query(&self, query: &Query) -> Result<QueryResult, String> {
        let database = self
            .active_database
            .as_ref()
            .ok_or_else(|| "No database selected".to_string())?;

        // Build the command
        let mut cmd = std::process::Command::new(&self.codeql_path);
        cmd.arg("query")
            .arg("run")
            .arg("--database")
            .arg(database)
            .arg("--format")
            .arg("sarif-latest")
            .arg("--threads")
            .arg(self.num_threads.to_string())
            .arg("--max-paths")
            .arg(self.max_paths.to_string())
            .arg("--timeout")
            .arg(query.timeout.to_string())
            .arg(&query.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Execute the query
        let start_time = std::time::Instant::now();
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute query: {}", e))?;
        let execution_time = start_time.elapsed();

        // Parse the results
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            // Try to parse SARIF results
            let sarif = serde_json::from_str::<SarifResults>(&stdout).ok();

            Ok(QueryResult {
                query_path: query.path.clone(),
                database_path: database.clone(),
                sarif,
                raw_output: stdout,
                execution_time,
                success: true,
                error_message: None,
            })
        } else {
            Ok(QueryResult {
                query_path: query.path.clone(),
                database_path: database.clone(),
                sarif: None,
                raw_output: stdout,
                execution_time,
                success: false,
                error_message: Some(stderr),
            })
        }
    }

    /// Performs quick evaluation of a predicate or expression
    pub async fn quick_evaluate(
        &self,
        predicate: &str,
        context_file: &Path,
        line: u32,
        column: u32,
    ) -> Result<String, String> {
        let database = self
            .active_database
            .as_ref()
            .ok_or_else(|| "No database selected".to_string())?;

        // Build the quick evaluation command
        let mut cmd = std::process::Command::new(&self.codeql_path);
        cmd.arg("query")
            .arg("eval")
            .arg("--database")
            .arg(database)
            .arg("--format")
            .arg("json")
            .arg("--quick-eval")
            .arg(format!("{}:{}:{}", context_file.display(), line, column))
            .arg(predicate)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to evaluate predicate: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Compiles a query to check for errors without executing
    pub async fn compile_query(&self, query_path: &Path) -> Result<(), String> {
        let mut cmd = std::process::Command::new(&self.codeql_path);
        cmd.arg("query")
            .arg("compile")
            .arg("--check-only")
            .arg(query_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to compile query: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Formats a query file using CodeQL's built-in formatter
    pub async fn format_query(&self, query_path: &Path) -> Result<String, String> {
        let mut cmd = std::process::Command::new(&self.codeql_path);
        cmd.arg("query")
            .arg("format")
            .arg("--in-place")
            .arg(query_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to format query: {}", e))?;

        if output.status.success() {
            // Read the formatted content
            std::fs::read_to_string(query_path)
                .map_err(|e| format!("Failed to read formatted query: {}", e))
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

/// Query history tracking
pub struct QueryHistory {
    entries: Vec<QueryHistoryEntry>,
    max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub query: Query,
    pub database: PathBuf,
    pub timestamp: String,
    pub execution_time: Duration,
    pub success: bool,
}

impl QueryHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn add_entry(&mut self, entry: QueryHistoryEntry) {
        self.entries.push(entry);

        // Keep only the most recent entries
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn get_recent(&self, count: usize) -> &[QueryHistoryEntry] {
        let start = self.entries.len().saturating_sub(count);
        &self.entries[start..]
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        std::fs::write(path, json).map_err(|e| format!("Failed to write history file: {}", e))
    }

    pub fn load_from_file(path: &Path, max_entries: usize) -> Result<Self, String> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read history file: {}", e))?;

        let entries: Vec<QueryHistoryEntry> =
            serde_json::from_str(&json).map_err(|e| format!("Failed to parse history: {}", e))?;

        Ok(Self {
            entries,
            max_entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_creation() {
        let query = Query {
            path: PathBuf::from("/path/to/query.ql"),
            metadata: Some(QueryMetadata {
                name: "Test Query".to_string(),
                description: Some("A test query".to_string()),
                severity: Some("warning".to_string()),
                tags: vec!["security".to_string()],
            }),
            timeout: 300,
        };

        assert_eq!(query.timeout, 300);
        assert!(query.metadata.is_some());
    }

    #[test]
    fn test_query_history() {
        let mut history = QueryHistory::new(10);

        let entry = QueryHistoryEntry {
            query: Query {
                path: PathBuf::from("/test.ql"),
                metadata: None,
                timeout: 60,
            },
            database: PathBuf::from("/test-db"),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            execution_time: Duration::from_secs(5),
            success: true,
        };

        history.add_entry(entry.clone());
        assert_eq!(history.entries.len(), 1);

        let recent = history.get_recent(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].timestamp, entry.timestamp);
    }
}
