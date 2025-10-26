use crate::core::{Config, ShadePaths, Tracker};
use crate::error::{Result, ShadeError};
use crate::utils::{detect_project_name, verify_git_repo};
use colored::Colorize;
use dialoguer::Confirm;
use std::fs;
use walkdir::WalkDir;

pub fn run(name_override: Option<String>) -> Result<()> {
    // 1. Verify it's a git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project name
    let project_name = detect_project_name(name_override)?;

    // 3. Setup paths
    let paths = ShadePaths::new()?;

    // 4. Verify shade repo exists
    if !paths.projects.join(".git").exists() {
        return Err(ShadeError::ShadeRepoNotFound);
    }

    // 5. Check if already initialized
    let mut config = Config::load(&paths.config)?;
    if config.find_project(&project_name).is_some() {
        return Err(ShadeError::AlreadyInitialized(project_name));
    }

    // 6. Create metadata directory
    paths.ensure_structure()?;
    let project_metadata_dir = paths.project_metadata_dir(&project_name);
    fs::create_dir_all(&project_metadata_dir)?;

    // 7. Create tracker file
    let tracker = Tracker::new();
    tracker.save(&paths.shade_sync_file(&project_name))?;

    // 8. Create project directory in shade
    let project_shade_dir = paths.project_shade_dir(&project_name);
    fs::create_dir_all(&project_shade_dir)?;

    // 9. Add to config
    config.add_project(project_name.clone(), project_path.clone())?;
    config.save(&paths.config)?;

    // 10. Print success
    println!(
        "{} Initialized git-shade for project: {}",
        "✓".green().bold(),
        project_name.bold()
    );
    println!("  Config: {}", paths.config.display());
    println!("  Metadata: {}", project_metadata_dir.display());
    println!("  Shade dir: {}", project_shade_dir.display());
    println!();

    // 11. Check if shade has files
    let existing_files = list_shade_files(&project_shade_dir)?;

    if !existing_files.is_empty() {
        println!("Found {} files in shade:", existing_files.len());
        for file in &existing_files {
            println!("  - {}", file.display());
        }
        println!();

        // Ask to pull
        let should_pull = Confirm::new()
            .with_prompt("Pull these files now?")
            .default(true)
            .interact()
            .map_err(|e| anyhow::anyhow!("Dialog error: {}", e))?;

        if should_pull {
            pull_files(&existing_files, &project_shade_dir, &project_path)?;
            add_to_exclude(&project_path, &existing_files, &project_shade_dir)?;
            println!();
            println!("{} Done!", "✓".green().bold());
        } else {
            println!("Skipped. Pull manually with: {}", "git-shade pull".bold());
        }
    }

    Ok(())
}

fn list_shade_files(shade_dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(shade_dir) {
        let entry = entry.map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?;
        if entry.file_type().is_file() {
            if let Ok(rel) = entry.path().strip_prefix(shade_dir) {
                files.push(rel.to_path_buf());
            }
        }
    }

    Ok(files)
}

fn pull_files(
    files: &[std::path::PathBuf],
    shade_dir: &std::path::Path,
    project_dir: &std::path::Path,
) -> Result<()> {
    use crate::utils::copy_file_preserve_structure;

    println!("Pulling files...");
    for file in files {
        let src = shade_dir.join(file);
        copy_file_preserve_structure(&src, shade_dir, project_dir)?;
        println!("  {} {}", "✓".green(), file.display());
    }

    Ok(())
}

fn add_to_exclude(
    project_dir: &std::path::Path,
    files: &[std::path::PathBuf],
    _shade_dir: &std::path::Path,
) -> Result<()> {
    use crate::git::add_to_exclude as git_add_to_exclude;

    let patterns: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    git_add_to_exclude(project_dir, &patterns)?;
    Ok(())
}
