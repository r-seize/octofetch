use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
        }
    }
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("octofetch").join("config.json"))
}

pub fn load() -> Config {
    let path = match config_path() {
        Some(p) => p,
        None => return Config::default(),
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Config::default(),
    };

    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path().ok_or_else(|| anyhow::anyhow!("Cannot find config directory"))?;

    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, content)?;

    println!("Config saved to {}", path.display());
    Ok(())
}

pub fn reset() -> Result<()> {
    save(&Config::default())
}

pub fn path_display() -> String {
    config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
