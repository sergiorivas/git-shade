use std::path::PathBuf;
use anyhow::{Context, Result};

pub struct ShadePaths {
    #[allow(dead_code)]  // Will be used for other operations
    pub root: PathBuf,
    pub config: PathBuf,
    pub metadata: PathBuf,
    pub projects: PathBuf,
}

// impl = implementation block (like Ruby's class methods)
impl ShadePaths {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        let root = home.join(".local/git-shade");

        Ok(Self {  // Self = ShadePaths (like @class in Ruby)
            config: root.join("config.toml"),
            metadata: root.join("metadata"),
            projects: root.join("projects"),
            root,
        })
    }

    // Instance method (&self = readonly access, like Ruby's regular method)
    pub fn ensure_structure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.metadata)
            .context("Failed to create metadata directory")?;
        Ok(())
    }

    // &self borrows the struct (doesn't consume it)
    pub fn project_metadata_dir(&self, project_name: &str) -> PathBuf {
        self.metadata.join(project_name)
    }

    pub fn project_shade_dir(&self, project_name: &str) -> PathBuf {
        self.projects.join(project_name)
    }

    pub fn shade_sync_file(&self, project_name: &str) -> PathBuf {
        self.project_metadata_dir(project_name).join(".shade-sync")
    }
}

#[cfg(test)]  // Only compiled for tests
mod tests {
    use super::*;

    #[test]
    fn test_paths_structure() {
        let paths = ShadePaths::new().unwrap();
        assert!(paths.root.ends_with(".local/git-shade"));
        assert!(paths.config.ends_with("config.toml"));
    }
}
