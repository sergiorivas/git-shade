use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tracker {
    pub last_pull: Option<DateTime<Utc>>,
    pub last_push: Option<DateTime<Utc>>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            last_pull: None,
            last_push: None,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let contents = std::fs::read_to_string(path)?;
        let tracker: Tracker = toml::from_str(&contents)?;
        Ok(tracker)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn update_pull(&mut self) {
        self.last_pull = Some(Utc::now());
    }

    pub fn update_push(&mut self) {
        self.last_push = Some(Utc::now());
    }
}
