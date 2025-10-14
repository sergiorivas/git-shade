use colored::Colorize;

pub fn run() {
    print_header();
    println!();
    print_what_is_git_shade();
    println!();
    print_how_it_works();
    println!();
    print_architecture();
    println!();
    print_first_time_setup();
    println!();
    print_daily_workflow();
    println!();
    print_commands_overview();
    println!();
    print_sync_states();
    println!();
    print_troubleshooting();
}

fn print_header() {
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_cyan());
    println!("{}", "                    git-shade User Guide                      ".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════════════".bright_cyan());
}

fn print_what_is_git_shade() {
    println!("{}", "What is git-shade?".bold().underline());
    println!();
    println!("git-shade is a CLI tool that syncs git-excluded files across machines.");
    println!();
    println!("{}:", "Problem".yellow().bold());
    println!("  Files in .gitignore or .git/info/exclude don't sync:");
    println!("  • Local config files (.env.local, config.local)");
    println!("  • API keys and secrets");
    println!("  • Large binary files");
    println!("  • Machine-specific settings");
    println!();
    println!("{}:", "Solution".green().bold());
    println!("  git-shade maintains a {} at:", "single unified Git repository".bold());
    println!("    ~/.local/git-shade/projects/");
    println!();
    println!("  This repository contains excluded files from {} your projects,", "ALL".bold());
    println!("  organized by project name, so they sync seamlessly across machines.");
}

fn print_how_it_works() {
    println!("{}", "How It Works".bold().underline());
    println!();
    println!("  1. You create/clone a {} Git repository", "single unified".bold());
    println!("     at ~/.local/git-shade/projects/");
    println!();
    println!("  2. For each project, git-shade:");
    println!("     • Creates a subdirectory: projects/<project-name>/");
    println!("     • Adds patterns to .git/info/exclude");
    println!("     • Syncs files to/from the shade directory");
    println!();
    println!("  3. The unified repo structure:");
    println!("     {}", "~/.local/git-shade/".bright_black());
    println!("       {}", "projects/                   # Single git repo".bright_black());
    println!("         {}", "myapp/                   # Your first project".green());
    println!("           {}", "config.local".bright_black());
    println!("           {}", "secrets/api.key".bright_black());
    println!("         {}", "another-project/         # Another project".green());
    println!("           {}", ".env.local".bright_black());
    println!("         {}", ".git/                    # One repo for all".cyan().bold());
}

fn print_architecture() {
    println!("{}", "Directory Architecture".bold().underline());
    println!();
    println!("  {} ~/.local/git-shade/", "Shade Storage:".cyan().bold());
    println!("    ├── config.toml           # Global configuration");
    println!("    ├── metadata/             # Per-project sync tracking");
    println!("    │   └── myapp/");
    println!("    │       └── .shade-sync   # Timestamps (last pull/push)");
    println!("    └── projects/             # {} Git repo", "Single unified".bold());
    println!("        ├── myapp/            # Files for project 1");
    println!("        │   ├── config.local");
    println!("        │   └── secrets/");
    println!("        ├── another-app/      # Files for project 2");
    println!("        │   └── .env.local");
    println!("        └── .git/             # {} for ALL projects", "One git repo".cyan().bold());
    println!();
    println!("  {} ~/projects/myapp/", "Your Project:".green().bold());
    println!("    ├── .git/                 # Main project git repo");
    println!("    │   └── info/exclude      # Patterns added by git-shade");
    println!("    ├── src/");
    println!("    ├── config.local          # {} Synced by git-shade", "←".yellow());
    println!("    └── secrets/              # {} Synced by git-shade", "←".yellow());
}

fn print_first_time_setup() {
    println!("{}", "First Time Setup".bold().underline());
    println!();
    println!("{}", "On Your First Machine:".green().bold());
    println!();
    println!("  {}", "Step 1: Create your unified shade repository".yellow());
    println!("  $ mkdir -p ~/.local/git-shade/projects");
    println!("  $ cd ~/.local/git-shade/projects");
    println!("  $ git init");
    println!("  $ git remote add origin git@github.com:you/my-shade-files.git");
    println!();
    println!("  {}", "Step 2: Initialize git-shade for your project".yellow());
    println!("  $ cd ~/projects/myapp");
    println!("  $ git-shade init");
    println!("  {} Initialized git-shade for project: myapp", "✓".green());
    println!();
    println!("  {}", "Step 3: Add files you want to sync".yellow());
    println!("  $ git-shade add config.local secrets/ .env.local");
    println!("  {} Added to .git/info/exclude", "✓".green());
    println!("  {} Copied to shade", "✓".green());
    println!();
    println!("  {}", "Step 4: Push to remote".yellow());
    println!("  $ git-shade push");
    println!("  {} Pushed to origin/main", "✓".green());
    println!();
    println!("{}", "On Your Second Machine:".blue().bold());
    println!();
    println!("  {}", "Step 1: Clone your unified shade repository".yellow());
    println!("  $ git clone git@github.com:you/my-shade-files.git \\");
    println!("      ~/.local/git-shade/projects");
    println!();
    println!("  {}", "Step 2: Clone your project and initialize".yellow());
    println!("  $ git clone git@github.com:you/myapp.git ~/projects/myapp");
    println!("  $ cd ~/projects/myapp");
    println!("  $ git-shade init");
    println!();
    println!("  {} git-shade will auto-detect existing files and ask:", "→".blue());
    println!("  Found 3 files in shade:");
    println!("    - config.local");
    println!("    - secrets/api.key");
    println!("    - .env.local");
    println!();
    println!("  Pull these files now? [Y/n]: {}", "y".green());
    println!("  {} Done! Files synced.", "✓".green());
}

fn print_daily_workflow() {
    println!("{}", "Daily Workflow".bold().underline());
    println!();
    println!("  {}", "Typical workflow on any machine:".cyan());
    println!();
    println!("  # Check what needs syncing");
    println!("  $ git-shade status");
    println!("    {} config.local (in sync)", "✓".green());
    println!("    {} secrets/api.key (local ahead)", "↑".yellow());
    println!("    {} .env.local (remote ahead)", "↓".blue());
    println!();
    println!("  # Pull remote changes");
    println!("  $ git-shade pull");
    println!("    {} Synced .env.local", "↓".blue());
    println!();
    println!("  # Make local changes");
    println!("  $ vim secrets/api.key");
    println!();
    println!("  # Push your changes");
    println!("  $ git-shade push");
    println!("    {} Pushed to origin/main", "✓".green());
    println!();
    println!("  {}", "On another machine:".cyan());
    println!();
    println!("  $ cd ~/projects/myapp");
    println!("  $ git-shade pull");
    println!("    {} Synced secrets/api.key", "↓".blue());
}

fn print_commands_overview() {
    println!("{}", "Commands Reference".bold().underline());
    println!();

    println!("  {} git-shade init [--name <name>]", "●".green());
    println!("    Initialize git-shade for current project");
    println!("    Auto-detects and offers to pull existing files");
    println!();

    println!("  {} git-shade add <files...>", "●".green());
    println!("    Add files/directories to shade");
    println!("    Automatically updates .git/info/exclude");
    println!("    Examples:");
    println!("      git-shade add config.local");
    println!("      git-shade add secrets/ .env.local");
    println!();

    println!("  {} git-shade push [-m \"message\"]", "●".green());
    println!("    Sync local changes to shade and push to remote");
    println!("    Uses automatic commit messages with hostname");
    println!();

    println!("  {} git-shade pull [--force] [--dry-run]", "●".green());
    println!("    Pull changes from shade to local project");
    println!("    Detects conflicts automatically");
    println!("    --force: Overwrite local without checking");
    println!("    --dry-run: Preview changes without applying");
    println!();

    println!("  {} git-shade status", "●".green());
    println!("    Show sync state of all tracked files");
    println!("    Displays helpful hints for next actions");
    println!();

    println!("  {} git-shade guide", "●".green());
    println!("    Show this guide (you're reading it now!)");
}

fn print_sync_states() {
    println!("{}", "Understanding Sync States".bold().underline());
    println!();
    println!("  git-shade compares file modification times to determine state:");
    println!();
    println!("  {} In Sync", "✓".green());
    println!("    Files are identical, no action needed");
    println!();
    println!("  {} Local Ahead", "↑".yellow());
    println!("    Modified locally after last pull");
    println!("    Action: Run {} to sync", "git-shade push".bold());
    println!();
    println!("  {} Remote Ahead", "↓".blue());
    println!("    Modified in shade after last pull");
    println!("    Action: Run {} to sync", "git-shade pull".bold());
    println!();
    println!("  {} Conflict", "⚠".red());
    println!("    Modified {} locally and remotely", "both".bold());
    println!("    Action: Manual resolution required");
    println!("    Options:");
    println!("      1. Review remote at ~/.local/git-shade/projects/<project>/");
    println!("      2. Manually merge and copy back");
    println!("      3. Use {} to take remote", "git-shade pull --force".bold());
    println!("      4. Use {} to take local", "git-shade push".bold());
    println!();
    println!("  {} Local Only", "?".bright_black());
    println!("    File exists locally but not in shade");
    println!("    (Probably just added, not pushed yet)");
    println!();
    println!("  {} Remote Only", "←".bright_black());
    println!("    File exists in shade but not locally");
    println!("    (Probably deleted locally, will be pulled)");
}

fn print_troubleshooting() {
    println!("{}", "Troubleshooting".bold().underline());
    println!();

    println!("  {} \"Not a git repository\"", "Q:".red().bold());
    println!("    You must run git-shade from inside a git repository.");
    println!("    Solution: cd to your project and ensure .git/ exists");
    println!();

    println!("  {} \"Shade repository not found\"", "Q:".red().bold());
    println!("    The unified shade repo doesn't exist yet.");
    println!("    Solution: Run the first time setup (see above)");
    println!();

    println!("  {} \"Project not initialized\"", "Q:".red().bold());
    println!("    This project hasn't been registered with git-shade.");
    println!("    Solution: Run {} first", "git-shade init".bold());
    println!();

    println!("  {} \"Conflicts detected\"", "Q:".red().bold());
    println!("    Files were modified on both machines since last sync.");
    println!("    Solution: See \"Understanding Sync States\" above");
    println!();

    println!("  {} How do I add more projects?", "Q:".cyan().bold());
    println!("    Just cd to the new project and run {}!", "git-shade init".bold());
    println!("    All projects share the same unified shade repository.");
    println!();

    println!("  {} Can I use git-shade with existing shade repos?", "Q:".cyan().bold());
    println!("    Yes! Just clone your existing shade repo to");
    println!("    ~/.local/git-shade/projects/ and run {} in projects.", "git-shade init".bold());
}
