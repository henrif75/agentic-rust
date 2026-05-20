//! Multi-Agent Research Assistant library.
//!
//! Exposes configuration, LLM adapters, mock search tool, and the core
//! orchestrator function to execute the multi-agent pipeline concurrently.

pub mod agent;
pub mod config;
pub mod tool;

use agent::LlmAdapter;
use config::AppConfig;
use std::sync::Arc;

/// Cleans markdown code blocks out of a JSON response.
pub fn clean_json_response(raw: &str) -> &str {
    let mut s = raw.trim();
    if s.starts_with("```json") {
        s = s.strip_prefix("```json").unwrap();
    } else if s.starts_with("```") {
        s = s.strip_prefix("```").unwrap();
    }
    if s.ends_with("```") {
        s = s.strip_suffix("```").unwrap();
    }
    s.trim()
}

/// Runs the multi-agent concurrent research pipeline:
/// Router -> Concurrent Research (MockSearch) -> Synthesizer.
pub async fn run_research_pipeline(
    adapter: Arc<dyn LlmAdapter>,
    config: &AppConfig,
    query: &str,
) -> Result<String, anyhow::Error> {
    // 1. Router Phase
    println!("--- ROUTER PHASE ---");
    let router_prompt = &config.prompts.router_prompt;
    let router_raw = adapter.prompt(router_prompt, query).await?;
    let cleaned = clean_json_response(&router_raw);

    let topics: Vec<String> = match serde_json::from_str(cleaned) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "Failed to parse router output as JSON list: {}. Raw response was: {}",
                e, router_raw
            );
            // Fall back to treating the query itself as the single topic
            vec![query.to_string()]
        }
    };

    println!("Router extracted sub-topics: {:?}", topics);
    if topics.is_empty() {
        return Err(anyhow::anyhow!("Router returned an empty list of topics."));
    }

    // 2. Concurrent Research Phase
    println!("\n--- CONCURRENT RESEARCH PHASE ---");
    let mut handles = vec![];
    for topic in topics {
        let adapter_clone = Arc::clone(&adapter);
        let research_prompt = config.prompts.research_prompt.clone();
        let handle = tokio::spawn(async move {
            println!("Starting research task for topic: '{}'", topic);
            let report = adapter_clone
                .prompt_with_search(&research_prompt, &topic)
                .await?;
            println!("Completed research task for topic: '{}'", topic);
            Ok::<String, anyhow::Error>(report)
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    let mut reports = vec![];
    for res in results {
        match res {
            Ok(Ok(report)) => reports.push(report),
            Ok(Err(e)) => return Err(anyhow::anyhow!("Research task failed: {}", e)),
            Err(e) => return Err(anyhow::anyhow!("Tokio join error: {}", e)),
        }
    }

    // 3. Synthesis Phase
    println!("\n--- SYNTHESIS PHASE ---");
    let synthesizer_prompt = &config.prompts.synthesizer_prompt;

    let mut synthesis_input = format!("Original User Query: {}\n\n", query);
    for (i, report) in reports.iter().enumerate() {
        synthesis_input.push_str(&format!(
            "--- Research Report {} ---\n{}\n\n",
            i + 1,
            report
        ));
    }

    let final_report = adapter.prompt(synthesizer_prompt, &synthesis_input).await?;
    Ok(final_report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_json_response() {
        assert_eq!(
            clean_json_response("```json\n[\"test\"]\n```"),
            "[\"test\"]"
        );
        assert_eq!(clean_json_response("  [\"test\"]  "), "[\"test\"]");
    }
}
