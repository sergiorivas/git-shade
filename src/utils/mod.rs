pub mod project;
pub mod fs;

pub use project::{detect_project_name, verify_git_repo};
pub use fs::{copy_file_preserve_structure, copy_dir_preserve_structure};
