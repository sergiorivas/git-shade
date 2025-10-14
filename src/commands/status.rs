use crate::core::{ShadePaths, Config, Tracker, FileMetadata, SyncState, detect_sync_state};
use crate::git::read_exclude;
use crate::utils::{verify_git_repo, detect_project_name};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use std::process::Command;

pub fn run() -> Result<()> {
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

    // 5. Load tracker
    let tracker = Tracker::load(&paths.shade_sync_file(&project_name))
        .unwrap_or_else(|_| Tracker::new());

    // 6. Print header
    println!("{}: {}", "Project".bold(), project_name);
    println!("{}: {}", "Local".bold(), project_path.display());
    println!("{}: {}", "Shade".bold(), project_shade_dir.display());
    
    if let Some(last_pull) = tracker.last_pull {
        println!("{}: {}", "Last pull".bold(), last_pull.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("{}: {}", "Last pull".bold(), "never".italic());
    }
    
    if let Some(last_push) = tracker.last_push {
        println!("{}: {}", "Last push".bold(), last_push.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("{}: {}", "Last push".bold(), "never".italic());
    }
    
    println!();

    // 7. Get tracked files
    let tracked_patterns = read_exclude(&project_path)?;
    
    if tracked_patterns.is_empty() {
        println!("No files tracked yet.");
        println!();
        println!("Add files with: {}", "git-shade add <files>".bold());
        return Ok(());
    }

    // 8. Analyze each tracked file
    println!("{}:", "Files".bold());
    
    let mut has_conflicts = false;
    let mut needs_push = false;
    let mut needs_pull = false;

    for pattern in &tracked_patterns {
        let clean_pattern = pattern.trim_end_matches('/');
        let local_path = project_path.join(clean_pattern);
        let shade_path = project_shade_dir.join(clean_pattern);

        // Get metadata
        let local_meta = if local_path.exists() && local_path.is_file() {
            Some(FileMetadata::from_path(&local_path).ok())
        } else {
            None
        }.flatten();

        let remote_meta = if shade_path.exists() && shade_path.is_file() {
            Some(FileMetadata::from_path(&shade_path).ok())
        } else {
            None
        }.flatten();

        // Detect state
        let state = detect_sync_state(
            local_meta.as_ref(),
            remote_meta.as_ref(),
            tracker.last_pull,
        );

        // Display with appropriate symbol and color
        let (symbol, description, color_fn): (_, _, fn(&str) -> colored::ColoredString) = match state {
            SyncState::InSync => {
                ("✓", "in sync", |s: &str| s.green())
            },
            SyncState::LocalAhead => {
                needs_push = true;
                ("↑", "local ahead - modified locally, ready to push", |s: &str| s.yellow())
            },
            SyncState::RemoteAhead => {
                needs_pull = true;
                ("↓", "remote ahead - modified in shade, safe to pull", |s: &str| s.blue())
            },
            SyncState::Conflict => {
                has_conflicts = true;
                ("⚠", "conflict - modified both locally and remotely", |s: &str| s.red())
            },
            SyncState::LocalOnly => {
                ("?", "local only, not in shade", |s: &str| s.bright_black())
            },
            SyncState::RemoteOnly => {
                needs_pull = true;
                ("←", "remote only, deleted locally", |s: &str| s.bright_black())
            },
        };

        println!("  {} {} ({})", color_fn(symbol), clean_pattern, description);
    }

    println!();

    // 9. Print legend
    println!("{}:", "Legend".bold());
    println!("  {} In sync           Both files are identical", "✓".green());
    println!("  {} Local ahead       Modified locally, needs push", "↑".yellow());
    println!("  {} Remote ahead      Modified in shade, safe to pull", "↓".blue());
    println!("  {} Conflict          Modified in both places, manual resolution needed", "⚠".red());
    println!("  {} Local only        File exists locally but not in shade", "?".bright_black());
    println!("  {} Remote only       File exists in shade but not locally", "←".bright_black());
    println!();

    // 10. Check git remote
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&paths.projects)?;

    let remote_output = Command::new("git")
        .args(["remote", "-v"])
        .output()?;

    let remote_status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    std::env::set_current_dir(&original_dir)?;

    if !remote_output.stdout.is_empty() {
        let remote_info = String::from_utf8_lossy(&remote_output.stdout);
        let first_line = remote_info.lines().next().unwrap_or("");
        if let Some(url) = first_line.split_whitespace().nth(1) {
            println!("{}: {}", "Git remote".bold(), url);
        }
    } else {
        println!("{}: {} - changes are local only", "Git remote".bold(), "(none)".italic());
        println!("  Add remote with:");
        println!("    cd {}", paths.projects.display());
        println!("    git remote add origin <url>");
        println!();
    }

    let is_clean = remote_status_output.stdout.is_empty();
    if is_clean {
        println!("{}: {} (no uncommitted changes)", "Git status".bold(), "Clean".green());
    } else {
        println!("{}: {} (uncommitted changes in shade)", "Git status".bold(), "Modified".yellow());
    }

    // 11. Provide helpful hints
    println!();
    if has_conflicts {
        println!("{} You have conflicts that need manual resolution.", "⚠".red().bold());
        println!("  Review files and run {} after resolving.", "git-shade push".bold());
    } else if needs_pull {
        println!("{} Some files can be pulled from shade.", "→".blue());
        println!("  Run {} to sync them.", "git-shade pull".bold());
    }
    
    if needs_push {
        println!("{} Some files have local changes.", "→".yellow());
        println!("  Run {} to sync them to shade.", "git-shade push".bold());
    }

    Ok(())
}
