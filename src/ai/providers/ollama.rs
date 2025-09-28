use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    ai::repository::{AIProvider, CommitMessage},
    error::{Error, Result},
};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaProvider {
    client: Client,
    ollama_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(ollama_url: String, model: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            ollama_url,
            model,
        })
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    async fn analyze_diff(&self, diff: &str) -> Result<CommitMessage> {
        let prompt = format!(
            "Analyze the following git diff and generate a **concise single-line commit message** in the format 'type(scope): subject':\n\n{}",
            diff
        );

        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.ollama_url))
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: OllamaResponse = serde_json::from_str(&response_text)
            .map_err(|e| Error::Ai(format!("Failed to parse Ollama response: {}. Raw response: {}", e, response_text)))?;

        let text = response_json.response.trim().to_string();

        // Basic parsing, assuming "type(scope): message"
        let mut parts = text.splitn(2, ':');
        let header = parts.next().unwrap_or("").trim();
        let message = parts.next().unwrap_or("").trim().to_string();

        let mut type_scope = header.splitn(2, '(');
        let commit_type = type_scope.next().unwrap_or("").to_string();
        let scope = type_scope
            .next()
            .and_then(|s| s.strip_suffix(')'))
            .map(|s| s.to_string());

        Ok(CommitMessage {
            commit_type,
            scope,
            message,
        })
    }
}
