//! Command line application entry point for the Multi-Agent Research Assistant.

use agentic_rust::{
    agent::build_adapter,
    config::{Cli, load_config, resolve_inputs},
    run_research_pipeline,
};
use clap::Parser;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env if present
    dotenvy::dotenv().ok();

    // Load configuration file
    let config = match load_config("config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "Warning: Could not load config.toml: {}. Using default configuration.",
                    e
                )
                .yellow()
            );
            agentic_rust::config::AppConfig::default()
        }
    };

    // Parse command line arguments
    let cli = Cli::parse();

    // Resolve inputs (interactive fallback)
    let (provider, query) = match resolve_inputs(cli, &config) {
        Ok(inputs) => inputs,
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    };

    println!(
        "{}",
        format!(
            "Initializing research assistant using provider: '{}'",
            provider
        )
        .green()
        .bold()
    );

    // Retrieve default model for the chosen provider
    let model = match provider.to_lowercase().as_str() {
        "gemini" => &config.models.gemini,
        "openai" => &config.models.openai,
        "anthropic" => &config.models.anthropic,
        "ollama" => &config.models.ollama,
        _ => {
            eprintln!(
                "{}",
                format!(
                    "Error: Unsupported provider '{}' in configuration.",
                    provider
                )
                .red()
            );
            std::process::exit(1);
        }
    };

    // Build adapter
    let adapter = match build_adapter(&provider, model) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", format!("Initialization failed: {}", e).red().bold());
            std::process::exit(1);
        }
    };

    println!(
        "{}",
        format!("Running research pipeline for: '{}'...", query).cyan()
    );

    match run_research_pipeline(adapter, &config, &query).await {
        Ok(report) => {
            println!(
                "\n{}",
                "================ FINAL REPORT ================"
                    .green()
                    .bold()
            );
            println!("{}", report);
            println!(
                "{}\n",
                "=============================================="
                    .green()
                    .bold()
            );
        }
        Err(e) => {
            eprintln!("{}", format!("Pipeline failed: {}", e).red().bold());
            std::process::exit(1);
        }
    }

    Ok(())
}
