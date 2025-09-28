use crate::error::Result;

/// Represents a structured commit message.
#[derive(Debug, PartialEq)]
pub struct CommitMessage {
    pub commit_type: String,
    pub scope: Option<String>,
    pub message: String,
}

/// A trait for AI providers that can analyze a diff and suggest a commit message.
pub trait AIProvider {
    /// Analyzes a git diff and returns a structured commit message.
    ///
    /// # Arguments
    ///
    /// * `diff` - A string slice that holds the git diff to be analyzed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CommitMessage`.
    fn analyze_diff(&self, diff: &str) -> Result<CommitMessage>;
}
