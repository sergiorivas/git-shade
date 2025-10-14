# git-shade

> Sync git-excluded files across multiple machines

**git-shade** is a CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files) so they sync seamlessly across machines while staying out of your main repository.

## Problem

Files in `.gitignore` or `.git/info/exclude` don't sync between machines:
- Local configuration files (`.env.local`, `config.local`)
- API keys and secrets
- Large binary files
- Machine-specific settings

You end up manually recreating these files on each machine. 😢

## Solution

git-shade creates a **single unified Git repository** at `~/.local/git-shade/projects/` that contains all your excluded files from all your projects, organized by project name.

```
~/.local/git-shade/projects/
  myapp/
    config.local
    secrets/api.key
  another-project/
    .env.local
  .git/  ← Single git repo for everything
```

## Features

✅ **Single unified repo** - One git repository for all projects  
✅ **Auto-excludes** - Automatically updates `.git/info/exclude`  
✅ **Conflict detection** - Warns when files modified on both machines  
✅ **Structure preserving** - Maintains directory hierarchy  
✅ **Interactive setup** - Auto-pulls existing files when initializing  
✅ **Status display** - See sync state of all files with colorful output  

## Installation

### From Source

```bash
cargo install --path .
```

Or build manually:

```bash
git clone https://github.com/yourusername/git-shade.git
cd git-shade
cargo build --release
sudo cp target/release/git-shade /usr/local/bin/
```

## Quick Start

### First Machine

```bash
# 1. Create and setup your unified shade repo (first time only)
mkdir -p ~/.local/git-shade/projects
cd ~/.local/git-shade/projects
git init
git remote add origin git@github.com:yourusername/my-shade-files.git

# 2. Initialize shade for your project
cd ~/projects/myapp
git-shade init

# 3. Add files you want to sync
git-shade add config.local secrets/ .env.local

# 4. Push to remote
git-shade push
```

### Second Machine

```bash
# 1. Clone your unified shade repo (contains ALL projects)
git clone git@github.com:yourusername/my-shade-files.git ~/.local/git-shade/projects

# 2. Clone your main project
git clone git@github.com:yourusername/myapp.git ~/projects/myapp
cd ~/projects/myapp

# 3. Initialize shade (auto-detects and offers to pull files)
git-shade init
# Found 3 files in shade:
#   - config.local
#   - secrets/api.key
#   - .env.local
#
# Pull these files now? [Y/n]: y
# ✓ Done!
```

## Commands

### `git-shade guide`

**New!** Get an interactive guide explaining how git-shade works.

```bash
git-shade guide
```

This command displays:
- What git-shade is and how it works
- Directory architecture diagram
- First-time setup instructions for both machines
- Daily workflow examples
- Commands reference
- Sync states explanation
- Troubleshooting guide

**Use this when:**
- You're new to git-shade
- You need to remember the setup process
- You want to understand sync states
- You're troubleshooting an issue

### `git-shade init [--name <name>]`

Initialize a project to use git-shade.

```bash
cd ~/projects/myapp
git-shade init
# ✓ Initialized git-shade for project: myapp
#   Config: ~/.local/git-shade/config.toml
#   Metadata: ~/.local/git-shade/metadata/myapp/
#   Shade dir: ~/.local/git-shade/projects/myapp/
```

**Flags:**
- `--name <name>` - Project name (default: current directory name)

### `git-shade add <files...>`

Add files or directories to shade.

```bash
# Add single file
git-shade add config.local

# Add multiple files
git-shade add config.local .env.local

# Add directory
git-shade add secrets/

# Add with paths
git-shade add src/config/database.yml
```

**Output:**
```
✓ Added to .git/info/exclude:
  - config.local
  - secrets/

✓ Copied to ~/.local/git-shade/projects/myapp/:
  - config.local
  - secrets/api.key
  - secrets/oauth.json

Ready to push with: git-shade push
```

### `git-shade push [-m <message>]`

Sync local changes to shade repo and push to remote.

```bash
git-shade push
# Copying files to shade...
#   ✓ config.local
#   ✓ secrets/api.key
#
# Git operations in ~/.local/git-shade/projects/...
#   ✓ Added: myapp/
#   ✓ Committed: [myapp] Update from macbook-pro - 2025-10-18 16:30:00
#   ✓ Pushed to origin/main
#
# Updated last_push: 2025-10-18T16:30:00Z
```

**Flags:**
- `-m, --message <msg>` - Custom commit message

### `git-shade pull [--force] [--dry-run]`

Pull changes from shade repo to local project.

```bash
git-shade pull
# Pulling from shade repo...
#   ✓ Git pull successful
#   Updated: myapp/, another-project/
#
# Checking for conflicts in myapp...
#   No conflicts detected
#
# Syncing files...
#   ↓ config.local (copied)
#   - secrets/api.key (in sync)
#
# Updated last_pull: 2025-10-18T16:35:00Z
#
# ✓ Pull completed successfully
```

**Flags:**
- `--force` - Overwrite local files without conflict checking
- `--dry-run` - Show what would happen without executing

**Conflict handling:**
```bash
git-shade pull
# ⚠ CONFLICTS DETECTED
#
# The following files were modified both locally and remotely:
#
#   ⚠ config.local
#     Local:  modified 2025-10-18 14:20:00
#     Remote: modified 2025-10-18 14:30:00
#
# Manual resolution required:
#   1. Go to ~/.local/git-shade/projects/myapp/
#   2. Review the remote versions
#   3. Choose which version to keep, OR manually merge
#   4. Copy resolved files to your project
#   5. OR use 'git-shade pull --force' to overwrite local with remote
```

### `git-shade status`

Show synchronization status of files.

```bash
git-shade status
# Project: myapp
# Local:      ~/projects/myapp
# Shade:      ~/.local/git-shade/projects/myapp/
# Last pull:  2025-10-18 14:00:00
# Last push:  2025-10-18 13:55:00
#
# Files:
#   ✓ config.local (in sync)
#   ↑ secrets/api.key (local ahead - modified locally, ready to push)
#   ↓ database.conf (remote ahead - modified in shade, safe to pull)
#   ⚠ private.key (conflict - modified both locally and remotely)
#
# Legend:
#   ✓ In sync           Both files are identical
#   ↑ Local ahead       Modified locally, needs push
#   ↓ Remote ahead      Modified in shade, safe to pull
#   ⚠ Conflict          Modified in both places, manual resolution needed
#
# Git remote: git@github.com:user/my-shade-files.git
# Git status: Clean (no uncommitted changes)
```

## Daily Workflow

```bash
# On Machine A - modify some config
vim config.local
git-shade push

# On Machine B - later
cd ~/projects/myapp
git-shade status  # Check what changed
git-shade pull     # Sync the changes
```

## How It Works

### Directory Structure

```
~/.local/git-shade/
  config.toml                # Global configuration
  metadata/                  # Per-project metadata
    myapp/
      .shade-sync           # Timestamps for myapp
  projects/                  # Single unified Git repo
    myapp/                  # Shade files for myapp
      config.local
      secrets/
        api.key
    another-project/        # Shade files for another project
      .env.local
    .git/                   # Single git repo for ALL projects
```

### Sync State Detection

git-shade compares file modification times to determine state:

| Condition | State | Action on Pull |
|-----------|-------|----------------|
| Local and remote unchanged since last_pull | `InSync` | Skip |
| Only local modified after last_pull | `LocalAhead` | Skip (you have newest) |
| Only remote modified after last_pull | `RemoteAhead` | Copy remote → local |
| Both modified after last_pull | `Conflict` | Error + instructions |
| File only in remote | `RemoteOnly` | Copy remote → local |
| File only in local | `LocalOnly` | Skip |

## Multiple Projects

The unified shade repo supports multiple projects:

```bash
# Project 1
cd ~/projects/myapp
git-shade init
git-shade add config.local

# Project 2
cd ~/projects/another-app
git-shade init
git-shade add .env.local

# Single push updates both
cd ~/.local/git-shade/projects
git add .
git commit -m "Update configs"
git push
```

Your shade repo structure:
```
~/.local/git-shade/projects/
  myapp/
    config.local
  another-app/
    .env.local
  .git/
```

## Configuration

### `~/.local/git-shade/config.toml`

```toml
version = "1.0"

[[projects]]
name = "myapp"
local_path = "/Users/username/projects/myapp"

[[projects]]
name = "another-project"
local_path = "/Users/username/work/another"
```

### `~/.local/git-shade/metadata/<project>/.shade-sync`

```toml
last_pull = "2025-10-18T14:30:00Z"
last_push = "2025-10-18T14:25:00Z"
```

## Troubleshooting

### "Shade repository not found"

You need to clone or create your unified shade repo first:

```bash
git clone git@github.com:yourusername/my-shade-files.git ~/.local/git-shade/projects
```

Or create a new one:

```bash
mkdir -p ~/.local/git-shade/projects
cd ~/.local/git-shade/projects
git init
git remote add origin git@github.com:yourusername/my-shade-files.git
```

### "Project not initialized"

Run `git-shade init` in your project directory first.

### Conflicts

When you have conflicts, git-shade will show you the conflicted files and their modification times. You have three options:

1. **Manual merge**: Go to `~/.local/git-shade/projects/<project>/`, review files, manually merge, copy to project, then `git-shade push`

2. **Keep local**: Just run `git-shade push` to overwrite remote with your local version

3. **Keep remote**: Run `git-shade pull --force` to overwrite local with remote version

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Run

```bash
cargo run -- init
cargo run -- add config.local
cargo run -- status
```

## Architecture

- **src/commands/** - Command implementations (init, add, push, pull, status)
- **src/core/** - Core logic (paths, config, tracker, sync state, conflicts)
- **src/git/** - Git operations (exclude file management)
- **src/utils/** - Utilities (fs operations, project detection)
- **src/error.rs** - Custom error types
- **src/cli.rs** - CLI argument parsing

## Dependencies

- `clap` - Command-line argument parsing
- `colored` - Terminal colors
- `serde` + `toml` - Configuration serialization
- `chrono` - Date/time handling
- `anyhow` + `thiserror` - Error handling
- `dialoguer` - Interactive prompts
- `walkdir` - Directory traversal
- `dirs` - Standard directory paths
- `hostname` - Machine hostname for commits

## License

MIT

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## Author

Created with ❤️ for developers who work across multiple machines.
