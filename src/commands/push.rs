use crate::core::{ShadePaths, Config, Tracker};
use crate::git::read_exclude;
use crate::utils::{verify_git_repo, detect_project_name, copy_file_preserve_structure, copy_dir_preserve_structure};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use std::process::Command;

pub fn run(message: Option<String>) -> Result<()> {
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

    // 5. Get tracked files from .git/info/exclude
    let patterns = read_exclude(&project_path)?;
    
    if patterns.is_empty() {
        return Err(ShadeError::NoFilesTracked);
    }

    // 6. Copy files from local to shade
    println!("Copying files to shade...");
    let mut copied_count = 0;

    for pattern in &patterns {
        // Remove trailing slash if it's a directory pattern
        let clean_pattern = pattern.trim_end_matches('/');
        let file_path = project_path.join(clean_pattern);

        if !file_path.exists() {
            println!("  {} {} (not found, skipped)", "⚠".yellow(), clean_pattern);
            continue;
        }

        if file_path.is_dir() {
            copy_dir_preserve_structure(&file_path, &project_path, &project_shade_dir)?;
        } else {
            copy_file_preserve_structure(&file_path, &project_path, &project_shade_dir)?;
        }

        println!("  {} {}", "✓".green(), clean_pattern);
        copied_count += 1;
    }

    if copied_count == 0 {
        println!("  No files copied (all tracked files are missing)");
        return Ok(());
    }

    println!();

    // 7. Git operations
    println!("Git operations in {}...", paths.projects.display());

    // Change to shade projects directory
    std::env::set_current_dir(&paths.projects)?;

    // Get hostname for commit message
    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // Build commit message
    let commit_msg = if let Some(msg) = message {
        format!("[{}] {}", project_name, msg)
    } else {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        format!("[{}] Update from {} - {}", project_name, hostname, timestamp)
    };

    // Git add (only this project's directory)
    let add_output = Command::new("git")
        .args(["add", &format!("{}/", project_name)])
        .output()?;

    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr);
        return Err(ShadeError::GitError(format!("git add failed: {}", stderr)));
    }

    println!("  {} Added: {}/", "✓".green(), project_name);

    // Git commit
    let commit_output = Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .output()?;

    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        
        // Check if it's "nothing to commit" (not an error)
        if stderr.contains("nothing to commit") || stderr.contains("no changes added") {
            println!("  {} No changes to commit", "→".blue());
        } else {
            return Err(ShadeError::GitError(format!("git commit failed: {}", stderr)));
        }
    } else {
        println!("  {} Committed: {}", "✓".green(), commit_msg);
    }

    // Check if remote exists
    let remote_output = Command::new("git")
        .args(["remote", "-v"])
        .output()?;

    let has_remote = !remote_output.stdout.is_empty();

    if has_remote {
        // Git push
        let push_output = Command::new("git")
            .args(["push"])
            .output()?;

        if !push_output.status.success() {
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            return Err(ShadeError::GitError(format!("git push failed: {}", stderr)));
        }

        println!("  {} Pushed to origin/main", "✓".green());
    } else {
        println!();
        println!("{} No remote configured. Changes saved locally only.", "⚠".yellow());
        println!("  To sync across machines, add a remote:");
        println!("    cd {}", paths.projects.display());
        println!("    git remote add origin <url>");
    }

    println!();

    // 8. Update tracker
    let mut tracker = Tracker::load(&paths.shade_sync_file(&project_name))
        .unwrap_or_else(|_| Tracker::new());
    tracker.update_push();
    tracker.save(&paths.shade_sync_file(&project_name))?;

    let timestamp = chrono::Utc::now().to_rfc3339();
    println!("Updated last_push: {}", timestamp);

    Ok(())
}
