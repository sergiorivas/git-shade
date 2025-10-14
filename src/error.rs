use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
#[allow(dead_code)]  // Some variants not yet used
pub enum ShadeError {
    #[error("Not a git repository: {path}\n\n\
             git-shade expects to be run from inside a git repository (your project).\n\n\
             Current directory: {path}\n\n\
             If this is your project directory, initialize git first:\n  \
             cd {path}\n  \
             git init\n\n\
             Then try git-shade again:\n  \
             git-shade init")]
    NotGitRepo {
        path: PathBuf,
    },

    #[error("Project not initialized: {project_name}\n\n\
             You need to initialize git-shade for this project first.\n\n\
             Run:\n  \
             git-shade init\n\n\
             This will:\n  \
             - Register your project in ~/.local/git-shade/config.toml\n  \
             - Create metadata directory\n  \
             - Set up sync tracking")]
    NotInitialized {
        project_name: String,
    },

    #[error("Project already initialized: {0}\n\n\
             This project is already set up with git-shade.\n\n\
             You can:\n  \
             - Add files: git-shade add <files>\n  \
             - Check status: git-shade status\n  \
             - Push changes: git-shade push\n  \
             - Pull changes: git-shade pull")]
    AlreadyInitialized(String),

    #[error("Shade repository not found\n\n\
             git-shade requires a unified shade repository at:\n  \
             ~/.local/git-shade/projects/\n\n\
             This repository doesn't exist yet. You need to set it up first.\n\n\
             FIRST TIME SETUP:\n\n\
             Option 1 - Clone existing shade repo (if you have one):\n  \
             git clone git@github.com:yourusername/my-shade-files.git ~/.local/git-shade/projects\n\n\
             Option 2 - Create new shade repo:\n  \
             mkdir -p ~/.local/git-shade/projects\n  \
             cd ~/.local/git-shade/projects\n  \
             git init\n  \
             git remote add origin git@github.com:yourusername/my-shade-files.git\n\n\
             Then try git-shade init again.")]
    ShadeRepoNotFound,

    #[error("File not found: {0}\n\n\
             The file or directory you're trying to add doesn't exist.\n\n\
             Make sure the path is correct and the file exists in your project.")]
    FileNotFound(PathBuf),

    #[error("No files tracked\n\n\
             This project has no files being tracked by git-shade.\n\n\
             Add files first:\n  \
             git-shade add <files>\n\n\
             Examples:\n  \
             git-shade add config.local\n  \
             git-shade add secrets/\n  \
             git-shade add .env.local database.yml")]
    NoFilesTracked,

    #[error("Conflicts detected. Manual resolution required.")]
    ConflictDetected {
        files: Vec<String>,
    },

    #[error("Git command failed: {0}")]
    GitError(String),

    // #[from] auto-converts std::io::Error → ShadeError
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// Type alias for cleaner return types
pub type Result<T> = std::result::Result<T, ShadeError>;
