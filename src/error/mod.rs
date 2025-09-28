use std::fmt;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the `gitsc` application.
#[derive(Debug)]
pub enum Error {
    /// An error originating from a Git operation.
    Git(String),
    /// An error from an I/O operation.
    Io(std::io::Error),
    /// An error from the AI provider.
    Ai(String),
    /// An error related to configuration.
    Config(String),
    /// An error for when there are no staged changes to analyze.
    NoStagedChanges,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(msg) => write!(f, "Git error: {}", msg),
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::Ai(msg) => write!(f, "AI provider error: {}", msg),
            Self::Config(msg) => write!(f, "Configuration error: {}", msg),
            Self::NoStagedChanges => write!(f, "No staged changes found to generate a commit message."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

// Convenience conversions from other error types.
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
