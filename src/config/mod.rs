use crate::error::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub cache_enabled: Option<bool>,
    pub cache_path: Option<PathBuf>,
    pub ollama_url: Option<String>,
    pub api_key: Option<String>,
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
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::Config("Could not find config directory".to_string()))?
        .join("gitsc");

    fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join("config.yml");

    if !config_path.exists() {
        const DEFAULT_CONFIG_CONTENT: &str = r#"
provider: gemini
model: "gemini-2.5-flash"
cache_enabled: true
cache_path: "~/.cache/gitsc/cache.db"
commit_format: "{type}({scope}): {message}"
log:
  path: "/tmp/gitsc.log"
  format: "nmap"
smart_commit:
  line_threshold: 150
"#;
        fs::write(&config_path, DEFAULT_CONFIG_CONTENT.trim())?;
    }

    let config_content = fs::read_to_string(&config_path)?;

    let config: Config = serde_yaml::from_str(&config_content)
        .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}
