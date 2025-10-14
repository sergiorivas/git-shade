use std::path::PathBuf;
use chrono::{DateTime, Utc};
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct ConflictInfo {
    pub file: PathBuf,
    pub local_modified: DateTime<Utc>,
    pub remote_modified: DateTime<Utc>,
    pub last_pull: DateTime<Utc>,
}

impl ConflictInfo {
    pub fn new(
        file: PathBuf,
        local_modified: DateTime<Utc>,
        remote_modified: DateTime<Utc>,
        last_pull: DateTime<Utc>,
    ) -> Self {
        Self {
            file,
            local_modified,
            remote_modified,
            last_pull,
        }
    }
}

/// Format conflict information into a user-friendly message
pub fn format_conflict_message(conflicts: &[ConflictInfo], shade_dir: &std::path::Path) -> String {
    let mut message = String::new();
    
    message.push_str(&format!("{} CONFLICTS DETECTED\n\n", "⚠".red().bold()));
    message.push_str("The following files were modified both locally and remotely since last pull:\n\n");
    
    for conflict in conflicts {
        message.push_str(&format!("  {} {}\n", "⚠".yellow(), conflict.file.display()));
        message.push_str(&format!(
            "    Local:  modified {} (after last pull at {})\n",
            conflict.local_modified.format("%Y-%m-%d %H:%M:%S"),
            conflict.last_pull.format("%Y-%m-%d %H:%M:%S")
        ));
        message.push_str(&format!(
            "    Remote: modified {} (after last pull at {})\n",
            conflict.remote_modified.format("%Y-%m-%d %H:%M:%S"),
            conflict.last_pull.format("%Y-%m-%d %H:%M:%S")
        ));
        message.push('\n');
    }
    
    message.push_str("Manual resolution required:\n");
    message.push_str(&format!("  1. Go to {}\n", shade_dir.display()));
    message.push_str("  2. Review the remote versions\n");
    message.push_str("  3. Choose which version to keep, OR manually merge\n");
    message.push_str("  4. Copy resolved files to your project\n");
    message.push_str(&format!("  5. OR use {} to overwrite local with remote\n", 
        "git-shade pull --force".bold()));
    message.push('\n');
    message.push_str("Aborted. No files were modified.");
    
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_format_conflict_message() {
        let conflicts = vec![
            ConflictInfo::new(
                PathBuf::from("config.local"),
                Utc::now(),
                Utc::now(),
                Utc::now() - chrono::Duration::hours(1),
            ),
        ];
        
        let message = format_conflict_message(&conflicts, &PathBuf::from("/test/shade"));
        
        assert!(message.contains("CONFLICTS DETECTED"));
        assert!(message.contains("config.local"));
        assert!(message.contains("Manual resolution required"));
    }
}
