use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Result};

/// Copy a file from source to destination, preserving directory structure
/// 
/// # Arguments
/// * `src` - Source file path
/// * `src_base` - Base directory for source (to calculate relative path)
/// * `dest_base` - Base directory for destination
pub fn copy_file_preserve_structure(
    src: &Path,
    src_base: &Path,
    dest_base: &Path,
) -> Result<PathBuf> {
    // Get relative path from source base
    let rel_path = src
        .strip_prefix(src_base)
        .context("Failed to calculate relative path")?;
    
    // Build destination path
    let dest = dest_base.join(rel_path);
    
    // Create parent directories if needed
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create parent directories")?;
    }
    
    // Copy the file
    fs::copy(src, &dest)
        .with_context(|| format!("Failed to copy {} to {}", src.display(), dest.display()))?;
    
    Ok(dest)
}

/// Copy entire directory recursively, preserving structure
pub fn copy_dir_preserve_structure(
    src_dir: &Path,
    src_base: &Path,
    dest_base: &Path,
) -> Result<Vec<PathBuf>> {
    let mut copied_files = Vec::new();
    
    for entry in walkdir::WalkDir::new(src_dir) {
        let entry = entry?;
        
        if entry.file_type().is_file() {
            let copied = copy_file_preserve_structure(
                entry.path(),
                src_base,
                dest_base,
            )?;
            copied_files.push(copied);
        }
    }
    
    Ok(copied_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_copy_file_preserve_structure() {
        let temp = TempDir::new().unwrap();
        let src_base = temp.path().join("src");
        let dest_base = temp.path().join("dest");
        
        // Create source file with nested structure
        let src_file = src_base.join("config/database.yml");
        fs::create_dir_all(src_file.parent().unwrap()).unwrap();
        fs::write(&src_file, "test content").unwrap();
        
        // Copy file
        let dest_file = copy_file_preserve_structure(&src_file, &src_base, &dest_base).unwrap();
        
        // Verify
        assert_eq!(dest_file, dest_base.join("config/database.yml"));
        assert!(dest_file.exists());
        assert_eq!(fs::read_to_string(&dest_file).unwrap(), "test content");
    }
    
    #[test]
    fn test_copy_dir_preserve_structure() {
        let temp = TempDir::new().unwrap();
        let src_base = temp.path().join("src");
        let dest_base = temp.path().join("dest");
        
        // Create source directory with files
        let secrets_dir = src_base.join("secrets");
        fs::create_dir_all(&secrets_dir).unwrap();
        fs::write(secrets_dir.join("api.key"), "secret1").unwrap();
        fs::write(secrets_dir.join("oauth.json"), "secret2").unwrap();
        
        // Copy directory
        let copied = copy_dir_preserve_structure(&secrets_dir, &src_base, &dest_base).unwrap();
        
        // Verify
        assert_eq!(copied.len(), 2);
        assert!(dest_base.join("secrets/api.key").exists());
        assert!(dest_base.join("secrets/oauth.json").exists());
    }
}
