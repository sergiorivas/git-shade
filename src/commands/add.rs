use crate::core::{Config, ShadePaths};
use crate::error::{Result, ShadeError};
use crate::git::add_to_exclude;
use crate::utils::{
    copy_dir_preserve_structure, copy_file_preserve_structure, detect_project_name, verify_git_repo,
};
use colored::Colorize;
use std::path::PathBuf;

pub fn run(files: Vec<PathBuf>) -> Result<()> {
    // 1. Verify it's a git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project name
    let project_name = detect_project_name(None)?;

    // 3. Setup paths
    let paths = ShadePaths::new()?;

    // 4. Verify project is initialized
    let config = Config::load(&paths.config)?;
    if config.find_project(&project_name).is_none() {
        return Err(ShadeError::NotInitialized { project_name });
    }

    let project_shade_dir = paths.project_shade_dir(&project_name);

    // 5. Process each file/directory
    let mut added_files = Vec::new();
    let mut patterns_to_exclude = Vec::new();

    for file_path in &files {
        let full_path = if file_path.is_absolute() {
            file_path.clone()
        } else {
            project_path.join(file_path)
        };

        // Verify file exists
        if !full_path.exists() {
            return Err(ShadeError::FileNotFound(file_path.clone()));
        }

        // Get relative path from project root
        let rel_path = full_path
            .strip_prefix(&project_path)
            .map_err(|_| anyhow::anyhow!("File is not inside project directory"))?;

        // Add to exclude patterns
        let pattern = if full_path.is_dir() {
            format!("{}/", rel_path.display())
        } else {
            rel_path.display().to_string()
        };
        patterns_to_exclude.push(pattern);

        // Copy to shade
        if full_path.is_dir() {
            let copied =
                copy_dir_preserve_structure(&full_path, &project_path, &project_shade_dir)?;
            added_files.extend(copied);
        } else {
            let copied =
                copy_file_preserve_structure(&full_path, &project_path, &project_shade_dir)?;
            added_files.push(copied);
        }
    }

    // 6. Add to .git/info/exclude
    add_to_exclude(&project_path, &patterns_to_exclude)?;

    // 7. Print success message
    println!("{} Added to .git/info/exclude:", "✓".green().bold());
    for pattern in &patterns_to_exclude {
        println!("  - {}", pattern);
    }
    println!();

    println!(
        "{} Copied to {}:",
        "✓".green().bold(),
        project_shade_dir.display()
    );

    // Show relative paths from shade dir
    for file in &added_files {
        if let Ok(rel) = file.strip_prefix(&project_shade_dir) {
            println!("  - {}", rel.display());
        }
    }
    println!();

    println!("Ready to push with: {}", "git-shade push".bold());

    Ok(())
}
