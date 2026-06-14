use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub cached_at: DateTime<Utc>,
}

impl<T: Serialize + for<'de> Deserialize<'de>> CacheEntry<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            cached_at: Utc::now(),
        }
    }

    pub fn is_fresh(&self, ttl_secs: i64) -> bool {
        let age = Utc::now().signed_duration_since(self.cached_at);
        age < Duration::seconds(ttl_secs)
    }
}

fn cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("octofetch"))
}

fn cache_path(key: &str) -> Option<PathBuf> {
    cache_dir().map(|d| d.join(format!("{}.json", sanitize(key))))
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn load<T: for<'de> Deserialize<'de>>(key: &str) -> Option<CacheEntry<T>> {
    let path       = cache_path(key)?;
    let content    = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn save<T: Serialize>(key: &str, entry: &CacheEntry<T>) -> Result<()> {
    if let Some(dir) = cache_dir() {
        std::fs::create_dir_all(&dir)?;
    }
    if let Some(path) = cache_path(key) {
        let content = serde_json::to_string_pretty(entry)?;
        std::fs::write(path, content)?;
    }
    Ok(())
}

pub fn clear(key: &str) {
    if let Some(path) = cache_path(key) {
        let _ = std::fs::remove_file(path);
    }
}

pub const TTL: i64 = 3600;
