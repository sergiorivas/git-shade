use crate::core::{ShadePaths, Config, Tracker, FileMetadata, SyncState, ConflictInfo, detect_sync_state, format_conflict_message};
use crate::git::{read_exclude, add_to_exclude};
use crate::utils::{verify_git_repo, detect_project_name, copy_file_preserve_structure};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use std::process::Command;
use walkdir::WalkDir;

pub fn run(force: bool, dry_run: bool) -> Result<()> {
    // 1. Verify it's a git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project name
    let project_name = detect_project_name(None)?;

    // 3. Setup paths
    let paths = ShadePaths::new()?;

    // 4. Verify project is initialized
    let config = Config::load(&paths.config)?;
    if config.find_project(&project_name).is_none() {
        return Err(ShadeError::NotInitialized {
            project_name,
        });
    }

    let project_shade_dir = paths.project_shade_dir(&project_name);

    // 5. Pull from git remote
    println!("Pulling from shade repo...");
    
    if !dry_run {
        // Change to shade projects directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&paths.projects)?;

        let pull_output = Command::new("git")
            .args(["pull"])
            .output()?;

        // Change back
        std::env::set_current_dir(&original_dir)?;

        if !pull_output.status.success() {
            let stderr = String::from_utf8_lossy(&pull_output.stderr);
            return Err(ShadeError::GitError(format!("git pull failed: {}", stderr)));
        }

        println!("  {} Git pull successful", "✓".green());
    } else {
        println!("  {} Git pull successful (dry-run)", "✓".green());
    }

    // Show which projects were updated
    let updated_projects = list_updated_projects(&paths.projects)?;
    if !updated_projects.is_empty() {
        print!("  Updated: ");
        for (i, proj) in updated_projects.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", proj);
        }
        println!();
    }
    println!();

    // 6. Load tracker to get last_pull time
    let tracker = Tracker::load(&paths.shade_sync_file(&project_name))
        .unwrap_or_else(|_| Tracker::new());
    let last_pull = tracker.last_pull;

    // 7. Get all files from shade directory
    let shade_files = list_all_files(&project_shade_dir)?;
    
    if shade_files.is_empty() {
        println!("No files in shade directory.");
        return Ok(());
    }

    // 8. Get tracked patterns from .git/info/exclude
    let tracked_patterns = read_exclude(&project_path)?;

    // 9. Analyze sync state for each file
    println!("Checking for conflicts in {}...", project_name);
    
    let mut conflicts = Vec::new();
    let mut files_to_sync = Vec::new();
    let mut files_to_add_to_exclude = Vec::new();

    for shade_file_path in &shade_files {
        let local_file_path = project_path.join(shade_file_path);
        
        // Get metadata
        let local_meta = if local_file_path.exists() {
            Some(FileMetadata::from_path(&local_file_path)?)
        } else {
            None
        };

        let shade_full_path = project_shade_dir.join(shade_file_path);
        let remote_meta = if shade_full_path.exists() {
            Some(FileMetadata::from_path(&shade_full_path)?)
        } else {
            None
        };

        // Detect state
        let state = detect_sync_state(
            local_meta.as_ref(),
            remote_meta.as_ref(),
            last_pull,
        );

        match state {
            SyncState::Conflict => {
                if !force {
                    conflicts.push(ConflictInfo::new(
                        shade_file_path.clone(),
                        local_meta.as_ref().unwrap().modified,
                        remote_meta.as_ref().unwrap().modified,
                        last_pull.unwrap(),
                    ));
                } else {
                    // Force mode: treat as remote ahead
                    files_to_sync.push((shade_file_path.clone(), "overwritten".to_string()));
                }
            },
            SyncState::RemoteAhead | SyncState::RemoteOnly => {
                files_to_sync.push((shade_file_path.clone(), "copied".to_string()));
                
                // Check if this file is tracked in exclude
                let pattern = shade_file_path.to_string_lossy().to_string();
                if !tracked_patterns.contains(&pattern) {
                    files_to_add_to_exclude.push(pattern);
                }
            },
            SyncState::InSync => {
                // No action needed
            },
            SyncState::LocalAhead | SyncState::LocalOnly => {
                // Skip - local is ahead or only exists locally
            },
        }
    }

    // 10. Handle conflicts
    if !conflicts.is_empty() && !force {
        println!();
        println!("{}", format_conflict_message(&conflicts, &project_shade_dir));
        return Err(ShadeError::ConflictDetected {
            files: conflicts.iter().map(|c| c.file.to_string_lossy().to_string()).collect(),
        });
    }

    if conflicts.is_empty() && !force {
        println!("  No conflicts detected");
    }

    println!();

    // 11. Sync files
    if files_to_sync.is_empty() {
        println!("All files are in sync. No changes needed.");
        return Ok(());
    }

    if force {
        println!("{} Force mode: overwriting all local files", "⚠".yellow());
    }

    println!("Syncing files...");

    for (file_path, action) in &files_to_sync {
        if !dry_run {
            let src = project_shade_dir.join(file_path);
            copy_file_preserve_structure(&src, &project_shade_dir, &project_path)?;
        }
        
        let symbol = if *action == "overwritten" { "✓" } else { "↓" };
        println!("  {} {} ({})", symbol.green(), file_path.display(), action);
    }

    // 12. Add new files to .git/info/exclude
    if !files_to_add_to_exclude.is_empty() && !dry_run {
        add_to_exclude(&project_path, &files_to_add_to_exclude)?;
        println!();
        println!("Updated .git/info/exclude");
    }

    // 13. Update tracker
    if !dry_run {
        let mut tracker = Tracker::load(&paths.shade_sync_file(&project_name))
            .unwrap_or_else(|_| Tracker::new());
        tracker.update_pull();
        tracker.save(&paths.shade_sync_file(&project_name))?;

        let timestamp = chrono::Utc::now().to_rfc3339();
        println!("Updated last_pull: {}", timestamp);
    } else {
        let timestamp = chrono::Utc::now().to_rfc3339();
        println!();
        println!("Would update last_pull: {}", timestamp);
    }

    println!();
    if dry_run {
        println!("{} Dry-run completed (no changes made)", "✓".blue());
    } else if force {
        println!("{} Pull completed (forced)", "✓".green().bold());
    } else {
        println!("{} Pull completed successfully", "✓".green().bold());
    }

    Ok(())
}

fn list_all_files(dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    if !dir.exists() {
        return Ok(files);
    }

    for entry in WalkDir::new(dir).min_depth(1) {
        let entry = entry.map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?;
        
        if entry.file_type().is_file() {
            if let Ok(rel) = entry.path().strip_prefix(dir) {
                files.push(rel.to_path_buf());
            }
        }
    }

    Ok(files)
}

fn list_updated_projects(projects_dir: &std::path::Path) -> Result<Vec<String>> {
    let mut updated = Vec::new();

    if !projects_dir.exists() {
        return Ok(updated);
    }

    for entry in std::fs::read_dir(projects_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            if let Some(name_str) = name.to_str() {
                if name_str != ".git" {
                    updated.push(name_str.to_string());
                }
            }
        }
    }

    Ok(updated)
}
