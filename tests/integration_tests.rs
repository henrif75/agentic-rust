//! Integration tests for the Multi-Agent Research Assistant.
//!
//! Tests the pipeline orchestrator and tool logic under various scenarios
//! using a mock adapter to ensure deterministic behavior without live network calls.

use elegant_pythagoras::{
    agent::{MockAdapter, build_adapter},
    config::{AppConfig, Cli, resolve_inputs},
    run_research_pipeline,
};
use std::env;
use std::sync::Arc;

/// Helper function to create a test configuration with mock prompts.
fn get_test_config() -> AppConfig {
    let models_struct = elegant_pythagoras::config::ModelsConfig {
        gemini: "gemini-1.5-flash".to_string(),
        openai: "gpt-4o-mini".to_string(),
        anthropic: "claude-3-5-sonnet".to_string(),
        ollama: "llama3".to_string(),
    };

    let prompts = elegant_pythagoras::config::PromptsConfig {
        router_prompt: "router system prompt".to_string(),
        research_prompt: "research system prompt".to_string(),
        synthesizer_prompt: "synthesizer system prompt".to_string(),
    };

    AppConfig {
        default_provider: "gemini".to_string(),
        models: models_struct,
        prompts,
    }
}

/// Scenario 1: Comparative Programming Language CLI Analysis
#[tokio::test]
async fn test_comparative_cli_analysis() {
    let config = get_test_config();

    // Mock adapter response
    let router_response = "[\"rust-cli\", \"go-cli\"]".to_string();
    let research_response = "Detailed technical capability analysis report...".to_string();
    let synthesizer_response =
        "Final synthesized comparative report of Rust vs Go CLIs".to_string();

    let adapter = Arc::new(MockAdapter {
        router_response,
        research_response,
        synthesizer_response: synthesizer_response.clone(),
    });

    let query = "Analyze CLI capabilities of Rust and Go";
    let final_report = run_research_pipeline(adapter, &config, query)
        .await
        .unwrap();

    assert_eq!(final_report, synthesizer_response);
}

/// Scenario 2: Database Technology Evaluation (three-way concurrency)
#[tokio::test]
async fn test_three_way_db_evaluation() {
    let config = get_test_config();

    // The router identifies three distinct sub-topics for concurrency
    let router_response = "[\"timescaledb\", \"influxdb\", \"clickhouse\"]".to_string();
    let research_response = "Specialized database capabilities report".to_string();
    let synthesizer_response = "Synthesized Time-Series database evaluation report".to_string();

    let adapter = Arc::new(MockAdapter {
        router_response,
        research_response,
        synthesizer_response: synthesizer_response.clone(),
    });

    let query = "Evaluate TimescaleDB vs InfluxDB vs ClickHouse";
    let final_report = run_research_pipeline(adapter, &config, query)
        .await
        .unwrap();

    assert_eq!(final_report, synthesizer_response);
}

/// Scenario 3: API Key Error Handling
#[test]
fn test_api_key_error_handling() {
    // Ensure the environment variable is not set during the test
    unsafe {
        env::remove_var("GEMINI_API_KEY");
    }

    // Build adapter should fail with an API key validation error
    let res = build_adapter("gemini", "gemini-1.5-flash");
    assert!(res.is_err());
    let err_msg = res.err().unwrap().to_string();
    assert!(err_msg.contains("GEMINI_API_KEY is not set"));
}

/// Scenario 4: CLI Parsing with Defaults and interactive fallback simulation
#[test]
fn test_cli_parsing_and_resolution() {
    let config = get_test_config();

    // Case A: Provider and query provided in arguments
    let cli = Cli {
        provider: Some("openai".to_string()),
        query: Some("Test query".to_string()),
    };

    let resolved = resolve_inputs(cli, &config);
    assert!(resolved.is_ok());
    let (provider, query) = resolved.unwrap();
    assert_eq!(provider, "openai");
    assert_eq!(query, "Test query");

    // Case B: No inputs provided, which triggers interactive dialoguer fallback.
    // Since dialoguer requires a TTY (which is absent in automated test runners),
    // resolve_inputs should return an error indicating that interactive input failed/aborted.
    let cli_empty = Cli {
        provider: None,
        query: None,
    };
    let resolved_empty = resolve_inputs(cli_empty, &config);
    assert!(resolved_empty.is_err());
    let err_str = resolved_empty.unwrap_err().to_string();
    assert!(err_str.contains("not a terminal") || err_str.contains("io"));
}
