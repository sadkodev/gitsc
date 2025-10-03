use clap::Parser;
use gitsc::ai::cache::diff_hasher;
use gitsc::ai::cache::{CacheRepository, sqlite_cache::SqliteCache};
use gitsc::ai::providers::{GeminiProvider, OllamaProvider};
use gitsc::ai::repository::AIProvider;
use gitsc::analyzer;
use gitsc::cli::Cli;
use gitsc::config::load_config;
use gitsc::error::Error;
use gitsc::formatter::format_commit_message;
use gitsc::git::{get_staged_diff, is_git_repository};
use gitsc::logger;
use log::{debug, error, info};
use serde_json;
use std::fs;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    logger::init(cli.debug);

    let config = load_config()?;

    if !is_git_repository() {
        error!("Error: Not a Git repository.");
        std::process::exit(1);
    }

    let diff = match get_staged_diff() {
        Ok(d) => d,
        Err(Error::NoStagedChanges) => {
            info!("No staged changes found.");
            return Ok(());
        }
        Err(e) => {
            error!("An error occurred: {}", e);
            std::process::exit(1);
        }
    };

    let processed_diff = analyzer::analyze_diff(&diff, &config.smart_commit);

    let cache: Option<Box<dyn CacheRepository + Send + Sync>> =
        if config.cache_enabled.unwrap_or(false) {
            if let Some(cache_path_str) = &config.cache_path {
                let expanded_cache_path = if let Some(s) = cache_path_str.to_str() {
                    if s.starts_with("~/") {
                        let home_dir = dirs::home_dir().ok_or_else(|| {
                            Error::Config(
                                "Could not find home directory for cache path expansion"
                                    .to_string(),
                            )
                        })?;
                        home_dir.join(&s[2..])
                    } else {
                        cache_path_str.clone()
                    }
                } else {
                    cache_path_str.clone()
                };

                if let Some(parent_dir) = expanded_cache_path.parent() {
                    fs::create_dir_all(parent_dir)?;
                }

                match SqliteCache::new(&expanded_cache_path).await {
                    Ok(c) => {
                        debug!(
                            "SQLite cache initialized successfully at {:?}",
                            expanded_cache_path
                        );
                        Some(Box::new(c))
                    }
                    Err(e) => {
                        error!(
                            "Failed to initialize SQLite cache at {:?}: {}",
                            expanded_cache_path, e
                        );
                        None
                    }
                }
            } else {
                error!("Cache enabled but no cache_path provided in config.");
                None
            }
        } else {
            debug!("Cache not enabled in config, skipping cache initialization.");
            None
        };

    let diff_hash = diff_hasher::generate_diff_hash(&processed_diff);
    let mut commit_message: Option<gitsc::ai::repository::CommitMessage> = None;

    if let Some(c) = &cache {
        match c.get(&diff_hash).await {
            Ok(Some(cached_message_str)) => {
                debug!("Cache hit for diff hash: {}", diff_hash);
                match serde_json::from_str(&cached_message_str) {
                    Ok(msg) => {
                        commit_message = Some(msg);
                        debug!("Commit message retrieved from cache.");
                    }
                    Err(e) => {
                        error!("Failed to deserialize cached commit message: {}", e);
                    }
                }
            }
            Ok(None) => {
                debug!("Cache miss for diff hash: {}", diff_hash);
            }
            Err(e) => {
                error!("Error retrieving from Redis cache: {}", e);
            }
        }
    }

    if commit_message.is_none() {
        let provider = match config.provider.as_str() {
            "gemini" => GeminiProvider::new(config.model.clone())
                .map(|p| Box::new(p) as Box<dyn AIProvider + Send + Sync>),
            "ollama" => {
                let ollama_url = config
                    .ollama_url
                    .clone()
                    .ok_or_else(|| Error::Config("Ollama URL not configured".to_string()))?;
                OllamaProvider::new(ollama_url, config.model.clone())
                    .map(|p| Box::new(p) as Box<dyn AIProvider + Send + Sync>)
            }
            _ => {
                error!("Error: Unsupported AI provider '{}'", config.provider);
                std::process::exit(1);
            }
        };

        match provider {
            Ok(p) => {
                debug!("Diff size: {} bytes", processed_diff.len());
                debug!(
                    "First 20 lines of diff:\n{}",
                    processed_diff
                        .lines()
                        .take(20)
                        .collect::<Vec<&str>>()
                        .join("\n")
                );
                debug!("Calling AI provider to analyze diff...");
                let start_time = Instant::now();
                match p.analyze_diff(&processed_diff).await {
                    Ok(msg) => {
                        let duration = start_time.elapsed();
                        debug!("AI provider responded in {:?}", duration);
                        if let Some(c) = &cache {
                            match serde_json::to_string(&msg) {
                                Ok(msg_str) => {
                                    if let Err(e) = c.set(&diff_hash, &msg_str).await {
                                        error!(
                                            "Failed to set cache for diff hash {}: {}",
                                            diff_hash, e
                                        );
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to serialize commit message for caching: {}", e);
                                }
                            }
                        }
                        commit_message = Some(msg);
                    }
                    Err(e) => {
                        error!("Error generating commit message: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                error!("Error creating AI provider: {}", e);
                std::process::exit(1);
            }
        }
    }

    if let Some(msg) = commit_message {
        let formatted_commit = format_commit_message(&msg, &config.commit_format);
        if cli.debug {
            info!("{}", formatted_commit);
        } else {
            println!("{}", formatted_commit);
        }
    } else {
        error!("Failed to generate or retrieve commit message.");
        std::process::exit(1);
    }

    Ok(())
}
