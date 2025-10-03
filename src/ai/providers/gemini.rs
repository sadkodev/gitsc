use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use crate::ai::repository::{AIProvider, CommitMessage};
use crate::error::{Error, Result};

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

pub struct GeminiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(model: String) -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| Error::Config("GEMINI_API_KEY not set".to_string()))?;
        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn analyze_diff(&self, diff: &str) -> Result<CommitMessage> {
        let prompt = format!(
            "Analyze the following git diff and generate a **concise single-line commit message** in the format 'type(scope): subject':\n\n{}",
            diff
        );

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = self
            .client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1/models/{}:generateContent?key={}",
                self.model, self.api_key
            ))
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: GeminiResponse = serde_json::from_str(&response_text).map_err(|e| {
            Error::Ai(format!(
                "Failed to parse Gemini response: {}. Raw response: {}",
                e, response_text
            ))
        })?;

        let text = response_json
            .candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| p.text.clone())
            .ok_or_else(|| Error::Ai("Failed to get commit message from Gemini".to_string()))?;

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
