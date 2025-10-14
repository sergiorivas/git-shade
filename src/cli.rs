use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-shade")]
#[command(about = "Sync git-excluded files across machines")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a project to use git-shade
    Init {
        #[arg(long, help = "Project name (default: current directory name)")]
        name: Option<String>,
    },
    /// Add files or directories to shade
    Add {
        #[arg(help = "Files or directories to add")]
        files: Vec<PathBuf>,
    },
    /// Sync local changes to shade repo and push
    Push {
        #[arg(short, long, help = "Custom commit message")]
        message: Option<String>,
    },
    /// Pull changes from shade repo to local project
    Pull {
        #[arg(long, help = "Overwrite local files without conflict checking")]
        force: bool,
        #[arg(long, help = "Show what would happen without executing")]
        dry_run: bool,
    },
    /// Show synchronization status of files
    Status,
    /// Explain how git-shade works and show setup guide
    Guide,
}
