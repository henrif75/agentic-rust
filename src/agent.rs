//! LLM Provider Adapters implementing the generic `LlmAdapter` trait.
//!
//! This module abstracts the differences between Gemini, OpenAI, Anthropic,
//! and Ollama clients, exposing a uniform, trait-based interface. It also
//! provides a `MockAdapter` for testability.

use crate::tool::MockSearch;
use async_trait::async_trait;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use std::env;
use std::sync::Arc;

/// Generic trait representing an LLM provider capable of prompt completion
/// and tool-integrated querying.
#[async_trait]
pub trait LlmAdapter: Send + Sync {
    /// Executes a simple text completion with a system prompt and user input.
    async fn prompt(&self, system_prompt: &str, user_prompt: &str)
    -> Result<String, anyhow::Error>;

    /// Executes a text completion with access to the Mock Search tool.
    async fn prompt_with_search(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error>;
}

/// Gemini adapter wrapping the Rig Gemini client.
pub struct GeminiAdapter {
    client: rig::providers::gemini::Client,
    model: String,
}

#[async_trait]
impl LlmAdapter for GeminiAdapter {
    async fn prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }

    async fn prompt_with_search(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .tool(MockSearch)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }
}

/// OpenAI adapter wrapping the Rig OpenAI client.
pub struct OpenaiAdapter {
    client: rig::providers::openai::Client,
    model: String,
}

#[async_trait]
impl LlmAdapter for OpenaiAdapter {
    async fn prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }

    async fn prompt_with_search(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .tool(MockSearch)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }
}

/// Anthropic adapter wrapping the Rig Anthropic client.
pub struct AnthropicAdapter {
    client: rig::providers::anthropic::Client,
    model: String,
}

#[async_trait]
impl LlmAdapter for AnthropicAdapter {
    async fn prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }

    async fn prompt_with_search(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .tool(MockSearch)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }
}

/// Ollama adapter wrapping the Rig Ollama client.
pub struct OllamaAdapter {
    client: rig::providers::ollama::Client,
    model: String,
}

#[async_trait]
impl LlmAdapter for OllamaAdapter {
    async fn prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }

    async fn prompt_with_search(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        let agent = self
            .client
            .agent(&self.model)
            .preamble(system_prompt)
            .tool(MockSearch)
            .build();
        let resp = agent.prompt(user_prompt).await?;
        Ok(resp)
    }
}

/// A Mock adapter that responds deterministically to simulate LLM agents in tests.
pub struct MockAdapter {
    /// Simulated router response (e.g. JSON array of sub-topics).
    pub router_response: String,
    /// Simulated research response.
    pub research_response: String,
    /// Simulated synthesis response.
    pub synthesizer_response: String,
}

#[async_trait]
impl LlmAdapter for MockAdapter {
    async fn prompt(
        &self,
        system_prompt: &str,
        _user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        if system_prompt.contains("coordinator") || system_prompt.contains("router") {
            Ok(self.router_response.clone())
        } else {
            Ok(self.synthesizer_response.clone())
        }
    }

    async fn prompt_with_search(
        &self,
        _system_prompt: &str,
        _user_prompt: &str,
    ) -> Result<String, anyhow::Error> {
        Ok(self.research_response.clone())
    }
}

/// Factory function to build an `LlmAdapter` depending on the provider name
/// and model configurations. It verifies environmental keys before construction.
pub fn build_adapter(provider: &str, model: &str) -> Result<Arc<dyn LlmAdapter>, anyhow::Error> {
    match provider.to_lowercase().as_str() {
        "gemini" => {
            if env::var("GEMINI_API_KEY").is_err() {
                return Err(anyhow::anyhow!(
                    "GEMINI_API_KEY is not set. Please set it in your environment or .env file."
                ));
            }
            let client = rig::providers::gemini::Client::from_env()?;
            Ok(Arc::new(GeminiAdapter {
                client,
                model: model.to_string(),
            }))
        }
        "openai" => {
            if env::var("OPENAI_API_KEY").is_err() {
                return Err(anyhow::anyhow!(
                    "OPENAI_API_KEY is not set. Please set it in your environment or .env file."
                ));
            }
            let client = rig::providers::openai::Client::from_env()?;
            Ok(Arc::new(OpenaiAdapter {
                client,
                model: model.to_string(),
            }))
        }
        "anthropic" => {
            if env::var("ANTHROPIC_API_KEY").is_err() {
                return Err(anyhow::anyhow!(
                    "ANTHROPIC_API_KEY is not set. Please set it in your environment or .env file."
                ));
            }
            let client = rig::providers::anthropic::Client::from_env()?;
            Ok(Arc::new(AnthropicAdapter {
                client,
                model: model.to_string(),
            }))
        }
        "ollama" => {
            let host =
                env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
            let client = rig::providers::ollama::Client::builder()
                .api_key("ollama")
                .base_url(&host)
                .build()?;
            Ok(Arc::new(OllamaAdapter {
                client,
                model: model.to_string(),
            }))
        }
        _ => Err(anyhow::anyhow!("Unsupported provider: {}", provider)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_adapter() {
        let adapter = MockAdapter {
            router_response: "[\"topic1\", \"topic2\"]".to_string(),
            research_response: "research output".to_string(),
            synthesizer_response: "summary report".to_string(),
        };

        let router_res = adapter
            .prompt("You are a router coordinator...", "query")
            .await
            .unwrap();
        assert_eq!(router_res, "[\"topic1\", \"topic2\"]");

        let research_res = adapter
            .prompt_with_search("research", "query")
            .await
            .unwrap();
        assert_eq!(research_res, "research output");

        let synth_res = adapter.prompt("synthesizer prompt", "query").await.unwrap();
        assert_eq!(synth_res, "summary report");
    }

    #[test]
    fn test_build_adapter_unsupported() {
        let res = build_adapter("unknown", "model");
        assert!(res.is_err());
    }
}
