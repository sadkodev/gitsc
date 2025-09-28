use clap::Parser;

/// A smart Git commit message generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Generate a single commit message for all changes.
    #[arg(short, long)]
    pub unique: bool,

    /// Split large diffs into smaller, meaningful commits (interactive mode).
    #[arg(short, long)]
    pub smart: bool,

    /// Apply a predefined or custom commit message format.
    #[arg(short, long)]
    pub format: Option<String>,

    /// Enable verbose output and developer debug logging.
    #[arg(short, long)]
    pub debug: bool,
}
