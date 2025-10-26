use anyhow::{Context, Result};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Add patterns to .git/info/exclude without creating duplicates
pub fn add_to_exclude(project_path: &Path, patterns: &[String]) -> Result<()> {
    let exclude_file = project_path.join(".git/info/exclude");

    // Ensure .git/info directory exists
    if let Some(parent) = exclude_file.parent() {
        fs::create_dir_all(parent).context("Failed to create .git/info directory")?;
    }

    // Read existing patterns
    let existing_patterns = if exclude_file.exists() {
        let file = fs::File::open(&exclude_file)?;
        BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>()
    } else {
        Vec::new()
    };

    // Filter out patterns that already exist
    let new_patterns: Vec<&String> = patterns
        .iter()
        .filter(|pattern| !existing_patterns.contains(pattern))
        .collect();

    if new_patterns.is_empty() {
        return Ok(());
    }

    // Append new patterns
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&exclude_file)?;

    for pattern in new_patterns {
        writeln!(file, "{}", pattern)?;
    }

    Ok(())
}

/// Read all patterns from .git/info/exclude
pub fn read_exclude(project_path: &Path) -> Result<Vec<String>> {
    let exclude_file = project_path.join(".git/info/exclude");

    if !exclude_file.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&exclude_file)?;
    let patterns = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .collect();

    Ok(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_add_to_exclude() {
        let temp = TempDir::new().unwrap();
        let project_path = temp.path();

        // Create .git directory
        fs::create_dir_all(project_path.join(".git/info")).unwrap();

        // Add patterns
        let patterns = vec!["config.local".to_string(), "secrets/".to_string()];
        add_to_exclude(project_path, &patterns).unwrap();

        // Verify patterns were added
        let result = read_exclude(project_path).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"config.local".to_string()));
        assert!(result.contains(&"secrets/".to_string()));

        // Add again (should not duplicate)
        add_to_exclude(project_path, &patterns).unwrap();
        let result = read_exclude(project_path).unwrap();
        assert_eq!(result.len(), 2);
    }
}
