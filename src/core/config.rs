use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: String,
    #[serde(default)] // If missing in TOML, use Vec::new()
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub local_path: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                version: "1.0".to_string(),
                projects: Vec::new(),
            });
        }

        let contents = std::fs::read_to_string(path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, contents).context("Failed to write config file")?;

        Ok(())
    }

    // &mut self = mutable borrow (can modify)
    pub fn add_project(&mut self, name: String, local_path: PathBuf) -> Result<()> {
        if self.projects.iter().any(|p| p.name == name) {
            anyhow::bail!("Project already exists: {}", name);
        }

        self.projects.push(Project { name, local_path });
        Ok(())
    }

    // Returns Option (like Ruby's nil, Go's nil, Elixir's nil)
    pub fn find_project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_save_and_load() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.toml");

        let mut config = Config {
            version: "1.0".to_string(),
            projects: Vec::new(),
        };

        config
            .add_project(
                "myapp".to_string(),
                PathBuf::from("/home/user/projects/myapp"),
            )
            .unwrap();

        config.save(&config_path).unwrap();

        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].name, "myapp");
    }
}
