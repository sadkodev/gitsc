use clap::Parser;
use gitsc::ai::providers::GeminiProvider;
use gitsc::ai::repository::AIProvider;
use gitsc::cli::Cli;
use gitsc::config::load_config;
use gitsc::error::Error;
use gitsc::formatter::format_commit_message;
use gitsc::git::{get_staged_diff, is_git_repository};
use gitsc::logger;
use log::{info, error, debug};
use gitsc::ai::cache::redis_cache::RedisCache;
use gitsc::ai::cache::diff_hasher;
use serde_json;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    logger::init(cli.debug);

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            error!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    if !is_git_repository() {
        error!("Error: Not a Git repository.");
        std::process::exit(1);
    }

    match get_staged_diff() {
        Ok(diff) => {
            if diff.is_empty() {
                info!("No staged changes found.");
                return;
            }

            let cache = if let Some(redis_url) = &config.redis_url {
                match RedisCache::new(redis_url) {
                    Ok(c) => {
                        debug!("Redis cache initialized successfully.");
                        Some(c)
                    },
                    Err(e) => {
                        error!("Failed to initialize Redis cache: {}", e);
                        None
                    }
                }
            } else {
                debug!("Redis URL not configured, skipping cache initialization.");
                None
            };

            let diff_hash = diff_hasher::generate_diff_hash(&diff);
            let mut commit_message: Option<gitsc::ai::repository::CommitMessage> = None;

            if let Some(c) = &cache {
                match c.get(&diff_hash).await {
                    Ok(Some(cached_message_str)) => {
                        debug!("Cache hit for diff hash: {}", diff_hash);
                        match serde_json::from_str(&cached_message_str) {
                            Ok(msg) => {
                                commit_message = Some(msg);
                                debug!("Commit message retrieved from cache.");
                            },
                            Err(e) => {
                                error!("Failed to deserialize cached commit message: {}", e);
                            }
                        }
                    },
                    Ok(None) => {
                        debug!("Cache miss for diff hash: {}", diff_hash);
                    },
                    Err(e) => {
                        error!("Error retrieving from Redis cache: {}", e);
                    }
                }
            }

            if commit_message.is_none() {
                let provider = match config.provider.as_str() {
                    "gemini" => GeminiProvider::new(config.model.clone())
                        .map(|p| Box::new(p) as Box<dyn AIProvider + Send + Sync>),
                    _ => {
                        error!("Error: Unsupported AI provider '{}'", config.provider);
                        std::process::exit(1);
                    }
                };

                match provider {
                    Ok(p) => {
                        debug!("Diff size: {} bytes", diff.len());
                        debug!("First 20 lines of diff:\n{}", diff.lines().take(20).collect::<Vec<&str>>().join("\n"));
                        debug!("Calling AI provider to analyze diff...");
                        let start_time = Instant::now();
                        match p.analyze_diff(&diff).await {
                            Ok(msg) => {
                                let duration = start_time.elapsed();
                                debug!("AI provider responded in {:?}", duration);
                                if let Some(c) = &cache {
                                    match serde_json::to_string(&msg) {
                                        Ok(msg_str) => {
                                            if let Err(e) = c.set(&diff_hash, &msg_str).await {
                                                error!("Failed to set cache for diff hash {}: {}", diff_hash, e);
                                            }
                                        },
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
                    },
                    Err(e) => {
                        error!("Error creating AI provider: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            if let Some(msg) = commit_message {
                let formatted_commit =
                    format_commit_message(&msg, &config.commit_format);
                if cli.debug {
                    info!("{}", formatted_commit);
                } else {
                    println!("{}", formatted_commit);
                }
            } else {
                error!("Failed to generate or retrieve commit message.");
                std::process::exit(1);
            }
        }
        Err(Error::NoStagedChanges) => {
            info!("No staged changes found.");
        }
        Err(e) => {
            error!("An error occurred: {}", e);
            std::process::exit(1);
        }
    }
}
