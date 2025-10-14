use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum SyncState {
    InSync,         // ✓ Files identical
    LocalAhead,     // ↑ Only local modified
    RemoteAhead,    // ↓ Only remote modified
    Conflict,       // ⚠ Both modified
    LocalOnly,      // ? Only exists locally
    RemoteOnly,     // ← Only exists remotely
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    #[allow(dead_code)]
    pub path: PathBuf,
    pub modified: DateTime<Utc>,
    pub size: u64,
}

impl FileMetadata {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let modified_utc: DateTime<Utc> = modified.into();
        
        Ok(Self {
            path: path.to_path_buf(),
            modified: modified_utc,
            size: metadata.len(),
        })
    }
}

/// Detect the sync state of a file by comparing local, remote, and last pull time
pub fn detect_sync_state(
    local_file: Option<&FileMetadata>,
    remote_file: Option<&FileMetadata>,
    last_pull: Option<DateTime<Utc>>,
) -> SyncState {
    match (local_file, remote_file, last_pull) {
        // File doesn't exist anywhere
        (None, None, _) => SyncState::InSync, // Shouldn't happen, but treat as in sync

        // Only exists locally
        (Some(_), None, _) => SyncState::LocalOnly,

        // Only exists remotely
        (None, Some(_), _) => SyncState::RemoteOnly,

        // Exists in both places
        (Some(local), Some(remote), Some(last_pull_time)) => {
            // If files are identical, they're in sync regardless of timestamps
            if local.modified == remote.modified && local.size == remote.size {
                return SyncState::InSync;
            }
            
            let local_modified_since_pull = local.modified > last_pull_time;
            let remote_modified_since_pull = remote.modified > last_pull_time;

            match (local_modified_since_pull, remote_modified_since_pull) {
                (false, false) => SyncState::InSync,
                (true, false) => SyncState::LocalAhead,
                (false, true) => SyncState::RemoteAhead,
                (true, true) => SyncState::Conflict,
            }
        },

        // Exists in both but never pulled before
        (Some(local), Some(remote), None) => {
            // Check if files are identical
            if local.modified == remote.modified && local.size == remote.size {
                SyncState::InSync
            } else {
                // First time, assume remote is source of truth
                SyncState::RemoteAhead
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_detect_sync_state_in_sync() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, "content").unwrap();
        
        let metadata = FileMetadata::from_path(&file).unwrap();
        
        // Both local and remote have same metadata, and last_pull is before the file was modified
        let last_pull = metadata.modified - chrono::Duration::seconds(10);
        
        let state = detect_sync_state(Some(&metadata), Some(&metadata), Some(last_pull));
        // Since both were modified after last_pull with identical times, it's actually in sync
        assert_eq!(state, SyncState::InSync);
    }

    #[test]
    fn test_detect_sync_state_local_only() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, "content").unwrap();
        
        let metadata = FileMetadata::from_path(&file).unwrap();
        
        let state = detect_sync_state(Some(&metadata), None, None);
        assert_eq!(state, SyncState::LocalOnly);
    }

    #[test]
    fn test_detect_sync_state_remote_only() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, "content").unwrap();
        
        let metadata = FileMetadata::from_path(&file).unwrap();
        
        let state = detect_sync_state(None, Some(&metadata), None);
        assert_eq!(state, SyncState::RemoteOnly);
    }

    #[test]
    fn test_detect_sync_state_conflict() {
        let temp = TempDir::new().unwrap();
        let local = temp.path().join("local.txt");
        let remote = temp.path().join("remote.txt");
        
        fs::write(&local, "local").unwrap();
        thread::sleep(Duration::from_millis(10));
        
        let last_pull = Utc::now();
        
        thread::sleep(Duration::from_millis(10));
        fs::write(&remote, "remote").unwrap();
        thread::sleep(Duration::from_millis(10));
        fs::write(&local, "local modified").unwrap();
        
        let local_meta = FileMetadata::from_path(&local).unwrap();
        let remote_meta = FileMetadata::from_path(&remote).unwrap();
        
        let state = detect_sync_state(Some(&local_meta), Some(&remote_meta), Some(last_pull));
        assert_eq!(state, SyncState::Conflict);
    }

    #[test]
    fn test_detect_sync_state_local_ahead() {
        let temp = TempDir::new().unwrap();
        let local = temp.path().join("local.txt");
        let remote = temp.path().join("remote.txt");
        
        fs::write(&local, "content").unwrap();
        fs::write(&remote, "content").unwrap();
        
        let last_pull = Utc::now();
        thread::sleep(Duration::from_millis(10));
        
        // Only modify local
        fs::write(&local, "modified").unwrap();
        
        let local_meta = FileMetadata::from_path(&local).unwrap();
        let remote_meta = FileMetadata::from_path(&remote).unwrap();
        
        let state = detect_sync_state(Some(&local_meta), Some(&remote_meta), Some(last_pull));
        assert_eq!(state, SyncState::LocalAhead);
    }

    #[test]
    fn test_detect_sync_state_remote_ahead() {
        let temp = TempDir::new().unwrap();
        let local = temp.path().join("local.txt");
        let remote = temp.path().join("remote.txt");
        
        fs::write(&local, "content").unwrap();
        fs::write(&remote, "content").unwrap();
        
        let last_pull = Utc::now();
        thread::sleep(Duration::from_millis(10));
        
        // Only modify remote
        fs::write(&remote, "modified").unwrap();
        
        let local_meta = FileMetadata::from_path(&local).unwrap();
        let remote_meta = FileMetadata::from_path(&remote).unwrap();
        
        let state = detect_sync_state(Some(&local_meta), Some(&remote_meta), Some(last_pull));
        assert_eq!(state, SyncState::RemoteAhead);
    }
}
