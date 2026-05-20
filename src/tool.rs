//! Mock search tool implementation for LLM agents.
//!
//! This module provides a simulated search tool that implements Rig's `Tool` trait.
//! It returns relevant snippets based on the keywords in the query.

use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Arguments for the search tool.
#[derive(Deserialize, Serialize, Debug)]
pub struct SearchArgs {
    /// The search query or keywords.
    pub query: String,
}

/// A simulated search tool that returns contextual answers based on keywords.
#[derive(Default, Clone, Debug)]
pub struct MockSearch;

/// Custom error type for the MockSearch tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolError(pub String);

impl std::error::Error for ToolError {}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tool error: {}", self.0)
    }
}

impl Tool for MockSearch {
    type Args = SearchArgs;
    type Error = ToolError;
    type Output = String;

    const NAME: &'static str = "mock_search";

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "mock_search".to_string(),
            description: "Search the web for up-to-date documentation, features, and comparative analysis of technical topics.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The technical topic or question to search for (e.g., 'Rust CLI clap crate', 'Go standard library CLI')"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let q = args.query.to_lowercase();
        println!("  [MockSearch] Searching for: '{}'", args.query);

        // Simulated database of search results
        let mut results: Vec<String> = Vec::new();

        if q.contains("rust") || q.contains("cargo") || q.contains("clap") {
            results.push("Rust CLI: The standard for argument parsing is the 'clap' crate (v4), supporting derive macros and builder APIs. Rust CLI tools compile to single, highly-optimized static binaries, making them fast and easy to distribute. Memory safety and zero-cost abstractions make Rust ideal for system-level command-line utility development.".to_string());
        }
        if q.contains("go") || q.contains("golang") {
            results.push("Go CLI: The Go standard library includes the 'flag' package for basic argument parsing. For advanced features, community crates like 'cobra' and 'viper' are standard. Go compiles extremely fast to statically linked binaries with a built-in garbage collector, making deployment trivial across multiple platforms.".to_string());
        }
        if q.contains("timescaledb") || q.contains("timescale") {
            results.push("TimescaleDB is built on PostgreSQL, utilizing automatic partitioning (hypertables) for time-series scalability. It supports standard SQL queries, full relational joins, and ACID compliance, making it excellent for time-series tracking combined with relational business data.".to_string());
        }
        if q.contains("influxdb") || q.contains("influx") {
            results.push("InfluxDB (specifically InfluxDB 3.0 / Clustered) is built in Rust using Apache Arrow and DataFusion. It uses the Flux querying language or SQL and is optimized for ultra-high-throughput log ingestion and real-time metric analytics, though it lacks relational support.".to_string());
        }
        if q.contains("clickhouse") {
            results.push("ClickHouse is an open-source, high-performance columnar database management system for online analytical processing (OLAP). It features massive parallel processing (MPP) and compression, enabling blazing-fast query execution over petabytes of time-series log data.".to_string());
        }
        if q.contains("postgres") || q.contains("postgresql") {
            results.push("PostgreSQL is a powerful, open-source object-relational database system with over 30 years of active development. It offers strong reliability, feature robustness, and ACID compliance, and supports extensions like TimescaleDB for time-series data.".to_string());
        }
        if q.contains("mysql") {
            results.push("MySQL is the world's most popular open-source relational database. It is widely used in web applications, offering high performance, ease of use, and integration with standard LAMP/LNMP stacks, but does not specialize in columnar or analytical scaling.".to_string());
        }

        if results.is_empty() {
            results.push(format!(
                "General search results for '{}': Found standard technical discussions and documentation references, showing general characteristics, active ecosystem support, and integration best practices.",
                args.query
            ));
        }

        Ok(results.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_search_rust() {
        let tool = MockSearch;
        let result = tool
            .call(SearchArgs {
                query: "Tell me about Rust CLI".to_string(),
            })
            .await
            .unwrap();
        assert!(result.contains("Rust CLI"));
        assert!(result.contains("clap"));
    }

    #[tokio::test]
    async fn test_mock_search_empty() {
        let tool = MockSearch;
        let result = tool
            .call(SearchArgs {
                query: "something obscure".to_string(),
            })
            .await
            .unwrap();
        assert!(result.contains("something obscure"));
    }
}
