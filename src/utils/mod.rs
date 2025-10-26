pub mod fs;
pub mod project;

pub use fs::{copy_dir_preserve_structure, copy_file_preserve_structure};
pub use project::{detect_project_name, verify_git_repo};
