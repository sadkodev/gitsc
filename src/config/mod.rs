use crate::error::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub commit_format: String,
    pub log: LogConfig,
    pub smart_commit: SmartCommitConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub path: PathBuf,
    pub format: LogFormat,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Nmap,
    Json,
}

#[derive(Debug, Deserialize)]
pub struct SmartCommitConfig {
    pub line_threshold: u32,
}

/// Loads the configuration. In debug builds, it loads from the current directory.
pub fn load_config() -> Result<Config> {
    let config_path = if cfg!(debug_assertions) {
        PathBuf::from("config.yml")
    } else {
        dirs::config_dir()
            .ok_or_else(|| Error::Config("Could not find config directory".to_string()))?
            .join("gitsc/config.yml")
    };

    if !config_path.exists() {
        return Err(Error::Config(format!(
            "Configuration file not found at: {}",
            config_path.display()
        )));
    }

    let config_content = fs::read_to_string(&config_path)?;

    let config: Config = serde_yaml::from_str(&config_content)
        .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}
