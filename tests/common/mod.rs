use std::path::PathBuf;
use tempfile::TempDir;

pub fn setup_test_repo() -> (TempDir, PathBuf) {
    let temp = TempDir::new().unwrap();
    let path = temp.path().to_path_buf();

    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(&path)
        .output()
        .unwrap();

    (temp, path)
}
