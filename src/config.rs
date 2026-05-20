//! Configuration and CLI argument parsing module.
//!
//! This module handles loading default settings (models, system prompts) from
//! `config.toml`, parsing command-line flags, and prompting the user interactively
//! if needed.

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Model configuration mapping providers to their frontier LLM models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelsConfig {
    /// Google Gemini model name.
    pub gemini: String,
    /// OpenAI model name.
    pub openai: String,
    /// Anthropic Claude model name.
    pub anthropic: String,
    /// Ollama local model name.
    pub ollama: String,
}

impl Default for ModelsConfig {
    fn default() -> Self {
        Self {
            gemini: "gemini-3.5-flash".to_string(),
            openai: "gpt-5.5-instant".to_string(),
            anthropic: "claude-4.6-sonnet".to_string(),
            ollama: "llama3.2".to_string(),
        }
    }
}

/// Prompt configuration for the three-tier multi-agent system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptsConfig {
    /// System prompt for the Router agent.
    pub router_prompt: String,
    /// System prompt for the Research sub-agents.
    pub research_prompt: String,
    /// System prompt for the Synthesizer agent.
    pub synthesizer_prompt: String,
}

impl Default for PromptsConfig {
    fn default() -> Self {
        Self {
            router_prompt: "You are a research coordinator. Analyze the query and break it down into exactly 2 to 4 independent sub-topic queries as a JSON array of strings.".to_string(),
            research_prompt: "You are an expert research analyst. Research the sub-topic and write a detailed report.".to_string(),
            synthesizer_prompt: "You are a professional editor. Merge the sub-topic reports into a cohesive comparative markdown report.".to_string(),
        }
    }
}

/// Complete application configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct AppConfig {
    /// Default provider to use if none is specified in CLI arguments.
    pub default_provider: String,
    /// Model name mappings.
    pub models: ModelsConfig,
    /// System prompts.
    pub prompts: PromptsConfig,
}

/// Command line interface parser powered by `clap`.
#[derive(Parser, Debug)]
#[command(name = "elegant-pythagoras")]
#[command(author = "Antigravity Authors")]
#[command(version = "1.0.0")]
#[command(about = "Concurrent Multi-Agent Research Assistant using Rig & Tokio", long_about = None)]
pub struct Cli {
    /// LLM provider to use (gemini, openai, anthropic, ollama)
    #[arg(short, long)]
    pub provider: Option<String>,

    /// The research query to run
    #[arg(short, long)]
    pub query: Option<String>,
}

/// Loads configuration from a TOML file.
/// If the file does not exist, it returns a default configuration.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<AppConfig, anyhow::Error> {
    if !path.as_ref().exists() {
        return Ok(AppConfig {
            default_provider: "gemini".to_string(),
            ..Default::default()
        });
    }

    let contents = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&contents)?;
    Ok(config)
}

/// Resolves the provider and query, utilizing CLI flags, config file defaults,
/// and falling back to interactive console prompts if inputs are missing.
pub fn resolve_inputs(cli: Cli, config: &AppConfig) -> Result<(String, String), anyhow::Error> {
    // 1. Resolve Provider
    let provider = if let Some(p) = cli.provider {
        p.to_lowercase()
    } else {
        // Run interactive prompt if not specified
        println!("No provider specified in arguments.");
        let choices = vec!["gemini", "openai", "anthropic", "ollama"];
        let selection = dialoguer::Select::new()
            .with_prompt("Select LLM Provider")
            .items(&choices)
            .default(
                choices
                    .iter()
                    .position(|&x| x == config.default_provider)
                    .unwrap_or(0),
            )
            .interact()?;
        choices[selection].to_string()
    };

    // 2. Resolve Query
    let query = if let Some(q) = cli.query {
        q
    } else {
        // Run interactive text input
        println!("No query specified in arguments.");
        let q: String = dialoguer::Input::new()
            .with_prompt("Enter your research query")
            .interact_text()?;
        q
    };

    Ok((provider, query))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_default() {
        let path = Path::new("non_existent_file.toml");
        let config = load_config(path).unwrap();
        assert_eq!(config.default_provider, "gemini");
        assert_eq!(config.models.gemini, "gemini-3.5-flash");
    }

    #[test]
    fn test_load_config_custom() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_data = r#"
            default_provider = "openai"
            [models]
            gemini = "gemini-test"
            openai = "openai-test"
            anthropic = "anthropic-test"
            ollama = "ollama-test"
            [prompts]
            router_prompt = "custom router"
            research_prompt = "custom research"
            synthesizer_prompt = "custom synthesizer"
        "#;
        write!(temp_file, "{}", toml_data).unwrap();

        let config = load_config(temp_file.path()).unwrap();
        assert_eq!(config.default_provider, "openai");
        assert_eq!(config.models.gemini, "gemini-test");
        assert_eq!(config.prompts.router_prompt, "custom router");
    }
}
