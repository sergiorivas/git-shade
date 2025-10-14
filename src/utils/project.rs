use std::path::PathBuf;
use std::env;
use crate::error::{Result, ShadeError};

pub fn detect_project_name(name_override: Option<String>) -> Result<String> {
    if let Some(name) = name_override {
        return Ok(name);
    }

    let current_dir = env::current_dir()?;

    // Get directory name
    let name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Could not determine project name"))?;

    Ok(name)
}

pub fn verify_git_repo() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let git_dir = current_dir.join(".git");

    if !git_dir.exists() {
        return Err(ShadeError::NotGitRepo {
            path: current_dir,
        });
    }

    Ok(current_dir)
}
