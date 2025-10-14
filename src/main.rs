mod cli;
mod error;
mod core;
mod utils;
mod commands;
mod git;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init::run(name),
        Commands::Add { files } => commands::add::run(files),
        Commands::Push { message } => commands::push::run(message),
        Commands::Pull { force, dry_run } => commands::pull::run(force, dry_run),
        Commands::Status => commands::status::run(),
        Commands::Guide => {
            commands::guide::run();
            Ok(())
        },
    }
}
