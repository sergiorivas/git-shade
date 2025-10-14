# PRD: git-shade v2.0

## 1. Overview

**Product:** git-shade - CLI tool to manage git-excluded files across multiple machines.

**Problem:** Files in `.gitignore` or `git/info/exclude` (local configs, secrets, large files) don't sync between machines, forcing manual recreation.

**Solution:** git-shade maintains a single unified Git repository in `~/.local/git-shade/projects/` that syncs excluded files from all your projects while keeping them out of each project's main repository.

**Language:** Rust

**Target:** Developers working across multiple machines who need to sync local/private files.

**Key Assumption:** Users will always start by cloning the unified shade repo:
```bash
git clone <url> ~/.local/git-shade/projects
```

---

## 2. Architecture

### 2.1 Directory Structure

```
~/.local/git-shade/                # All shade data
  config.toml                      # Global configuration
  metadata/                        # Tracking metadata per project
    myapp/
      .shade-sync                  # Timestamps for myapp
    another-project/
      .shade-sync                  # Timestamps for another-project
  projects/                        # Single unified Git repository
    myapp/                         # Shade files for myapp
      config.local
      secrets/
        api.key
    another-project/               # Shade files for another-project
      .env.local
      private.conf
    .git/                          # Single git repo for ALL projects

~/projects/myapp/                  # Normal project (example)
  src/
  .git/                            # Main project git repo
  .gitignore
  config.local                     # ← Synced by shade
  secrets/                         # ← Synced by shade
    api.key
```

**Key differences from previous design:**
- **Single Git repository** for all projects at `~/.local/git-shade/projects/.git/`
- Each project has its own subdirectory within `projects/`
- Simpler setup: one `git clone` covers all your shade files

### 2.2 Data Flow

```
Local Project (~/projects/myapp/)
    ↓ add
.git/info/exclude (adds entry to exclude from main git)
    ↓ push (copies files)
~/.local/git-shade/projects/myapp/ (in unified git repo)
    ↓ git commit + push (entire projects/ repo)
Remote Git Repository (your unified shade repo)
    ↓ git pull (entire projects/ repo, other machine)
~/.local/git-shade/projects/myapp/ (updated)
    ↓ pull (copies files)
Local Project (other machine)
```

**Important:**
- All projects share ONE Git repository
- Each project has its own subdirectory
- Commits can include changes from multiple projects
- Pull/push operations update ALL projects at once

---

## 3. Commands

### 3.1 `git-shade init`

**Purpose:** Initialize a project to use git-shade

**Behavior:**
1. Verifies current directory is a git repository (`.git/` exists)
2. Detects project name (from current directory or `--name` flag)
3. Verifies `~/.local/git-shade/projects/.git/` exists (user must clone first)
4. Creates structure in `~/.local/git-shade/`:
   - `metadata/<project>/.shade-sync`
   - `projects/<project>/` (directory for this project's files)
5. Registers project in `~/.local/git-shade/config.toml`
6. Checks if `~/.local/git-shade/projects/<project>/` has files:
   - If yes: Lists files found
   - Asks if user wants to auto-pull
   - If accepted: copies files shade → local + adds to `.git/info/exclude`
   - If declined: shows informational message

**Syntax:**
```bash
git-shade init [--name <name>]
```

**Flags:**
- `--name <name>`: Project name (default: current directory name)

**Setup Workflow (First Time Ever):**
```bash
# 1. Create and clone your unified shade repo
# (Do this ONCE, not per project)
cd ~/.local
git clone git@github.com:user/my-shade-files.git git-shade/projects

# 2. Initialize shade for a project
cd ~/projects/myapp
git-shade init
# ✓ Initialized git-shade for project: myapp
#   Add files with: git-shade add <files>
```

**Setup Workflow (Second Machine):**
```bash
# 1. Clone unified shade repo (contains ALL projects)
git clone git@github.com:user/my-shade-files.git ~/.local/git-shade/projects

# 2. Clone main project
git clone git@github.com:user/myapp.git ~/projects/myapp

# 3. Initialize shade
cd ~/projects/myapp
git-shade init
# ✓ Initialized git-shade for project: myapp
#
# Found 3 files in shade:
#   - config.local
#   - secrets/api.key
#   - .env.local
#
# Pull these files now? [Y/n]: y
#
# Pulling files...
#   ✓ config.local
#   ✓ secrets/api.key
#   ✓ .env.local
#
# Updated .git/info/exclude
# ✓ Done!
```

**Output (first project, no files):**
```
✓ Initialized git-shade for project: myapp
  Config: ~/.local/git-shade/config.toml
  Metadata: ~/.local/git-shade/metadata/myapp/
  Shade dir: ~/.local/git-shade/projects/myapp/

  Add files with: git-shade add <files>
```

**Output (files exist in shade):**
```
✓ Initialized git-shade for project: myapp

Found 3 files in shade:
  - config.local
  - secrets/api.key
  - .env.local

Pull these files now? [Y/n]: y

Pulling files...
  ✓ config.local
  ✓ secrets/api.key
  ✓ .env.local

Updated .git/info/exclude

✓ Done!
```

**Errors:**
- Not a git repository: `Not a git repository. Run 'git init' first.`
- Already initialized: `Project already initialized.`
- Shade repo not cloned: `Shade repository not found. Clone it first with: git clone <url> ~/.local/git-shade/projects`

---

### 3.2 `git-shade add <files...>`

**Purpose:** Add files or directories to shade

**Behavior:**
1. Verifies project is initialized
2. For each specified path:
   - Verifies file/directory exists locally
   - Adds path to `.git/info/exclude` (no duplicates)
   - Copies file/directory to `~/.local/git-shade/projects/<project>/`
   - Preserves directory structure relative to project root
3. Shows summary of added files

**Syntax:**
```bash
git-shade add <file1> [file2] [dir/] ...
```

**Examples:**
```bash
# Add single file
git-shade add config.local

# Add multiple files
git-shade add config.local .env.local

# Add entire directory
git-shade add secrets/

# Add with relative paths
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

**Directory behavior:**
- `secrets/` copies entire directory recursively
- Maintains structure: `secrets/api.key` → `~/.local/git-shade/projects/myapp/secrets/api.key`

**Errors:**
- File doesn't exist: `File not found: config.local`
- Project not initialized: `Project not initialized. Run 'git-shade init' first.`
- Not a git repo: `Not a git repository.`

**Notes:**
- Doesn't commit or push automatically, just prepares files
- Can add files multiple times without duplicating exclude entries
- If file already in shade, overwrites with local version

---

### 3.3 `git-shade push`

**Purpose:** Sync local changes to unified shade repo and remote

**Behavior:**
1. Verifies project is initialized
2. Copies all tracked files (listed in `.git/info/exclude`) from local → `~/.local/git-shade/projects/<project>/`
3. In unified repo (`~/.local/git-shade/projects/`):
   - `git add <project>/` (only adds this project's changes)
   - Creates commit with message (custom or automatic)
   - If remote configured: `git push`
   - If no remote: warns it's local only
4. Updates `last_push` timestamp in `~/.local/git-shade/metadata/<project>/.shade-sync`

**Syntax:**
```bash
git-shade push [-m <message>]
```

**Flags:**
- `-m, --message <msg>`: Custom commit message

**Output (normal):**
```
Copying files to shade...
  ✓ config.local
  ✓ secrets/api.key

Git operations in ~/.local/git-shade/projects/...
  ✓ Added: myapp/config.local, myapp/secrets/api.key
  ✓ Committed: [myapp] Update from macbook-pro
  ✓ Pushed to origin/main

Updated last_push: 2025-10-11T14:30:00Z
```

**Output (no remote):**
```
Copying files to shade...
  ✓ config.local
  ✓ secrets/api.key

Git operations in ~/.local/git-shade/projects/...
  ✓ Committed: [myapp] Update from macbook-pro

⚠ No remote configured. Changes saved locally only.
  To sync across machines, add a remote:
    cd ~/.local/git-shade/projects
    git remote add origin <url>

Updated last_push: 2025-10-11T14:30:00Z
```

**Automatic commit message:**
- Default: `"[<project>] Update from <hostname> - <timestamp>"`
- With `-m`: `"[<project>] <custom message>"`
- Project name prefix helps identify which project changed

**Errors:**
- Project not initialized: `Project not initialized.`
- No tracked files: `No files to push. Use 'git-shade add' first.`
- Git push fails: `Git push failed: <error>`

**Notes:**
- Only stages files from current project (`git add myapp/`)
- Other projects' changes are unaffected
- Multiple projects can be committed separately or together

---

### 3.4 `git-shade pull`

**Purpose:** Pull changes from unified shade repo to local project

**Behavior:**
1. Verifies project is initialized
2. Runs `git pull` in `~/.local/git-shade/projects/` (updates ALL projects)
3. For each file in `~/.local/git-shade/projects/<project>/`:
   - Gets metadata (modification timestamp)
   - Compares with local file (if exists)
   - Compares with `last_pull` timestamp from `.shade-sync`
   - Determines state (see table below)
4. If CONFLICTS detected:
   - Shows list of conflicted files
   - Shows manual resolution instructions
   - Aborts without copying (unless `--force`)
5. If NO conflicts:
   - Copies files with `RemoteAhead` state from shade → local
   - Skips files with `LocalAhead` or `InSync` state
   - Automatically adds new files to `.git/info/exclude`
6. Updates `last_pull` timestamp in `.shade-sync`

**State Detection:**

| Condition | State | Action on Pull |
|-----------|-------|----------------|
| Local and remote unchanged since last_pull | `InSync` | Skip |
| Only local modified after last_pull | `LocalAhead` | Skip (you have newest) |
| Only remote modified after last_pull | `RemoteAhead` | Copy remote → local |
| Both modified after last_pull | `Conflict` | Error + instructions |
| File only in remote | `RemoteOnly` | Copy remote → local |
| File only in local | `LocalOnly` | Skip |
| No last_pull (first time) | `RemoteAhead` | Copy remote → local |

**Syntax:**
```bash
git-shade pull [--force] [--dry-run]
```

**Flags:**
- `--force`: Overwrite local files without conflict checking
- `--dry-run`: Show what would happen without executing

**Output (no conflicts):**
```
Pulling from shade repo...
  ✓ Git pull successful
  Updated: myapp/, another-project/

Checking for conflicts in myapp...
  No conflicts detected

Syncing files...
  ↓ config.local (remote newer, copied)
  - secrets/api.key (in sync)
  ↑ .env.local (local newer, skipped)

Updated .git/info/exclude
Updated last_pull: 2025-10-11T14:35:00Z

✓ Pull completed successfully
```

**Output (with conflicts):**
```
Pulling from shade repo...
  ✓ Git pull successful

Checking for conflicts in myapp...

⚠ CONFLICTS DETECTED

The following files were modified both locally and remotely since last pull:

  ⚠ config.local
    Local:  modified 2025-10-11 14:20:00 (after last pull at 13:00:00)
    Remote: modified 2025-10-11 14:30:00 (after last pull at 13:00:00)

  ⚠ secrets/api.key
    Local:  modified 2025-10-11 14:25:00
    Remote: modified 2025-10-11 14:28:00

Manual resolution required:
  1. Go to ~/.local/git-shade/projects/myapp/
  2. Review the remote versions
  3. Choose which version to keep, OR manually merge
  4. Copy resolved files to your project
  5. OR use 'git-shade pull --force' to overwrite local with remote

Aborted. No files were modified.
```

**Output (with `--force`):**
```
Pulling from shade repo...
  ✓ Git pull successful

⚠ Force mode: overwriting all local files

Syncing files...
  ✓ config.local (overwritten)
  ✓ secrets/api.key (overwritten)
  ✓ .env.local (overwritten)

Updated last_pull: 2025-10-11T14:40:00Z

✓ Pull completed (forced)
```

**Output (with `--dry-run`):**
```
Pulling from shade repo...
  ✓ Git pull successful (dry-run)

Checking for conflicts...
  No conflicts detected

Would sync files:
  ↓ config.local (would copy)
  - secrets/api.key (would skip, in sync)
  ↑ .env.local (would skip, local newer)

Would update last_pull: 2025-10-11T14:42:00Z

✓ Dry-run completed (no changes made)
```

**Errors:**
- Project not initialized: `Project not initialized.`
- Conflicts detected: See output above
- Git pull fails: `Git pull failed: <error>`

**Notes:**
- `git pull` updates ALL projects in the unified repo
- Only analyzes/syncs current project's files
- Other projects are updated in shade but not synced to local

---

### 3.5 `git-shade status`

**Purpose:** Show synchronization status of files

**Behavior:**
1. Verifies project is initialized
2. Reads tracked files from `.git/info/exclude`
3. For each file:
   - Detects state by comparing local vs shade vs last_pull
   - Shows with appropriate symbol and color
4. Shows last sync info (pull/push)
5. Shows legend

**Syntax:**
```bash
git-shade status
```

**Output:**
```
Project: myapp
Local:      ~/projects/myapp
Shade:      ~/.local/git-shade/projects/myapp/
Last pull:  2025-10-11 14:00:00
Last push:  2025-10-11 13:55:00

Files:
  ✓ config.local (in sync)
  ↑ secrets/api.key (local ahead - modified locally, ready to push)
  ↓ database.conf (remote ahead - modified in shade, safe to pull)
  ⚠ private.key (conflict - modified both locally and remotely)
  ? new-file.txt (local only, not in shade)
  ← old-config.yml (remote only, deleted locally)

Legend:
  ✓ In sync           Both files are identical
  ↑ Local ahead       Modified locally, needs push
  ↓ Remote ahead      Modified in shade, safe to pull
  ⚠ Conflict          Modified in both places, manual resolution needed
  ? Local only        File exists locally but not in shade
  ← Remote only       File exists in shade but not locally

Git remote: git@github.com:user/my-shade-files.git
Git status: Clean (no uncommitted changes)
```

**With colors:**
- `✓` green
- `↑` yellow
- `↓` blue
- `⚠` red
- `?` gray
- `←` gray

**Special cases:**

**If no files tracked:**
```
Project: myapp
No files tracked yet.

Add files with: git-shade add <files>
```

**If no git remote:**
```
Project: myapp
...
Git remote: (none) - changes are local only
  Add remote with:
    cd ~/.local/git-shade/projects
    git remote add origin <url>
```

---

### 3.6 `git-shade list` (Optional)

**Purpose:** List all projects configured in shade

**Syntax:**
```bash
git-shade list
```

**Output:**
```
Configured projects:

  myapp
    Local:  ~/projects/myapp
    Shade:  ~/.local/git-shade/projects/myapp/
    Status: ✓ Active (3 files tracked)

  another-project
    Local:  ~/work/another
    Shade:  ~/.local/git-shade/projects/another-project/
    Status: ✓ Active (5 files tracked)

  old-project
    Local:  ~/projects/old (directory not found)
    Shade:  ~/.local/git-shade/projects/old-project/
    Status: ✗ Local directory missing

Unified shade repo: ~/.local/git-shade/projects/
Git remote: git@github.com:user/my-shade-files.git

Total: 3 projects
```

---

## 4. File Formats

### 4.1 `~/.local/git-shade/config.toml`

```toml
# Configuration format version
version = "1.0"

# List of projects
[[projects]]
name = "myapp"
local_path = "/Users/username/projects/myapp"

[[projects]]
name = "another-project"
local_path = "/Users/username/work/another"
```

**Fields:**
- `version`: Format version (for future migrations)
- `projects[].name`: Unique project name (must match directory in `projects/`)
- `projects[].local_path`: Absolute path to local project

**Note:** Shade path is implicit: `~/.local/git-shade/projects/<name>/`

### 4.2 `~/.local/git-shade/metadata/<project>/.shade-sync`

```toml
# Timestamp of last successful pull
last_pull = "2025-10-11T14:30:00Z"

# Timestamp of last successful push
last_push = "2025-10-11T14:25:00Z"
```

**Timestamp format:** ISO 8601 with UTC timezone

**Note:** Only contains timestamps. File hashes aren't stored; states are calculated in real-time by comparing `mtime` (modification time) from filesystem.

---

## 5. Conflict Detection - Detailed Algorithm

### 5.1 Detection Logic

```rust
fn detect_sync_state(
    local_file: Option<FileMetadata>,
    remote_file: Option<FileMetadata>,
    last_pull: Option<DateTime<Utc>>,
) -> SyncState {
    match (local_file, remote_file, last_pull) {
        // File doesn't exist anywhere
        (None, None, _) => SyncState::NotTracked,

        // Only exists locally
        (Some(_), None, _) => SyncState::LocalOnly,

        // Only exists remotely
        (None, Some(_), _) => SyncState::RemoteOnly,

        // Exists in both places
        (Some(local), Some(remote), Some(last_pull_time)) => {
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
            if local.modified == remote.modified && local.size == remote.size {
                SyncState::InSync
            } else {
                // First time, assume remote is truth
                SyncState::RemoteAhead
            }
        },
    }
}
```

### 5.2 Possible States

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum SyncState {
    InSync,         // ✓ Files identical
    LocalAhead,     // ↑ Only local modified
    RemoteAhead,    // ↓ Only remote modified
    Conflict,       // ⚠ Both modified
    LocalOnly,      // ? Only exists locally
    RemoteOnly,     // ← Only exists remotely
    NotTracked,     // (internal, not displayed)
}
```

### 5.3 File Metadata

```rust
struct FileMetadata {
    path: PathBuf,
    modified: DateTime<Utc>,  // Last modification timestamp
    size: u64,                 // Size in bytes
}
```

**Timestamp source:** `fs::metadata(path).modified()` from OS

---

## 6. Complete Use Cases

### 6.1 Setup on First Machine

```bash
# 1. You have an existing git project
cd ~/projects/myapp

# 2. Create unified shade repo (first time EVER)
mkdir -p ~/.local/git-shade/projects
cd ~/.local/git-shade/projects
git init
git remote add origin git@github.com:user/my-shade-files.git

# 3. Initialize shade for this project
cd ~/projects/myapp
git-shade init
# ✓ Initialized git-shade for project: myapp

# 4. Add files you want to sync
git-shade add config.local secrets/ .env.local
# ✓ Added to .git/info/exclude
# ✓ Copied to shade

# 5. Check status
git-shade status
# Files:
#   ↑ config.local (local ahead)
#   ↑ secrets/api.key (local ahead)

# 6. Push to remote
git-shade push
# ✓ Committed: [myapp] Update from macbook
# ✓ Pushed to origin/main

# Done! Files are synced
```

### 6.2 Setup on Second Machine

```bash
# 1. Clone unified shade repo (contains ALL projects)
git clone git@github.com:user/my-shade-files.git ~/.local/git-shade/projects

# 2. Clone main project
git clone git@github.com:user/myapp.git ~/projects/myapp
cd ~/projects/myapp

# 3. Initialize shade (auto-detects files)
git-shade init
# ✓ Initialized git-shade for project: myapp
#
# Found 3 files in shade:
#   - config.local
#   - secrets/api.key
#   - .env.local
#
# Pull these files now? [Y/n]: y
#
# ✓ Files synced
# ✓ Updated .git/info/exclude

# Done! You have all private files
```

### 6.3 Daily Workflow

```bash
# On Machine A - Modify config.local
vim config.local

# Push changes
git-shade push
# ✓ Pushed to origin/main

# On Machine B - Later
cd ~/projects/myapp
git-shade pull
# ✓ Pull completed
#   ↓ config.local (remote newer, copied)

# Or check first
git-shade status
# Files:
#   ↓ config.local (remote ahead)

git-shade pull
```

### 6.4 Conflict Resolution

```bash
# Machine A
vim config.local  # Modified at 14:20
git-shade push

# Machine B - meanwhile you also modified it
vim config.local  # Modified at 14:25

# Try to pull
git-shade pull
# ⚠ CONFLICTS DETECTED
#   ⚠ config.local
#     Local:  modified 2025-10-11 14:25:00
#     Remote: modified 2025-10-11 14:30:00
#
# Manual resolution required...

# Option 1: Manual merge
cd ~/.local/git-shade/projects/myapp
vim config.local  # Review remote version
# Merge manually, then copy back
cp config.local ~/projects/myapp/
cd ~/projects/myapp
git-shade push  # Your merged version wins

# Option 2: Force overwrite with remote
git-shade pull --force
# ✓ config.local (overwritten)

# Option 3: Keep local, overwrite remote
git-shade push  # Your local version wins
```

### 6.5 Multiple Projects in Same Shade

```bash
# Setup second project
cd ~/projects/another-app
git-shade init
git-shade add .env.local database.yml
git-shade push

# Now your unified shade repo contains:
# ~/.local/git-shade/projects/
#   myapp/
#     config.local
#     secrets/
#   another-app/
#     .env.local
#     database.yml
#   .git/

# Pulling updates all projects
cd ~/projects/myapp
git-shade pull
# ✓ Git pull successful
#   Updated: myapp/, another-app/
#
# Syncing files for myapp...
#   - config.local (in sync)
```

---

## 7. Error Handling

### 7.1 Common Errors and Messages

| Situation | Command | Message | Exit Code |
|-----------|---------|---------|-----------|
| Not a git repo | `init` | `Not a git repository. Run 'git init' first.` | 1 |
| Already initialized | `init` | `Project already initialized: myapp` | 1 |
| Not initialized | `add`, `push`, `pull`, `status` | `Project not initialized. Run 'git-shade init' first.` | 1 |
| Shade repo not cloned | `init` | `Shade repository not found. Clone it first: git clone <url> ~/.local/git-shade/projects` | 1 |
| File doesn't exist | `add` | `File not found: config.local` | 1 |
| No tracked files | `push`, `pull` | `No files tracked. Use 'git-shade add' first.` | 1 |
| Conflicts on pull | `pull` | See section 3.4 | 2 |
| Git command fails | `push`, `pull` | `Git command failed: <stderr>` | 3 |
| Permissions | various | `Permission denied: <path>` | 1 |

### 7.2 Error Types in Rust

```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum ShadeError {
    #[error("Not a git repository. Run 'git init' first.")]
    NotGitRepo,

    #[error("Project not initialized. Run 'git-shade init' first.")]
    NotInitialized,

    #[error("Project already initialized: {0}")]
    AlreadyInitialized(String),

    #[error("Shade repository not found at ~/.local/git-shade/projects/\nClone it first with: git clone <url> ~/.local/git-shade/projects")]
    ShadeRepoNotFound,

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("No files tracked. Use 'git-shade add' first.")]
    NoFilesTracked,

    #[error("Conflicts detected. Manual resolution required.")]
    ConflictDetected {
        files: Vec<String>,
        instructions: String,
    },

    #[error("Git command failed: {0}")]
    GitError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ShadeError>;
```

---

## 8. Dependencies (Cargo.toml)

```toml
[package]
name = "git-shade"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Sync git-excluded files across machines"
license = "MIT"
repository = "https://github.com/yourusername/git-shade"

[dependencies]
# CLI
clap = { version = "4.5", features = ["derive", "cargo"] }
colored = "2.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# File system
walkdir = "2.5"
fs_extra = "1.3"

# Paths
dirs = "5.0"

# User interaction
dialoguer = "0.11"  # For confirm() prompts

# Logging (optional for debugging)
env_logger = "0.11"
log = "0.4"

[dev-dependencies]
tempfile = "3.10"
assert_cmd = "2.0"
predicates = "3.1"
serial_test = "3.0"  # For tests that can't run in parallel

[profile.release]
strip = true
lto = true
codegen-units = 1
opt-level = "z"  # Optimize for size
```

---

## 9. Code Structure

```
git-shade/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs              # Entry point + logging setup
│   ├── cli.rs               # Command definitions with clap
│   ├── error.rs             # Custom error types
│   │
│   ├── commands/            # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs          # git-shade init
│   │   ├── add.rs           # git-shade add
│   │   ├── push.rs          # git-shade push
│   │   ├── pull.rs          # git-shade pull
│   │   ├── status.rs        # git-shade status
│   │   └── list.rs          # git-shade list (optional)
│   │
│   ├── core/                # Core logic
│   │   ├── mod.rs
│   │   ├── config.rs        # Read/write config.toml
│   │   ├── tracker.rs       # Read/write .shade-sync
│   │   ├── sync.rs          # File synchronization logic
│   │   ├── conflict.rs      # Conflict detection
│   │   └── paths.rs         # ShadePaths struct
│   │
│   ├── git/                 # Git wrapper
│   │   ├── mod.rs
│   │   ├── repo.rs          # GitRepo struct with operations
│   │   └── exclude.rs       # Handle .git/info/exclude
│   │
│   └── utils/               # Utilities
│       ├── mod.rs
│       ├── fs.rs            # File system helpers
│       ├── prompt.rs        # User interaction (confirm, etc)
│       └── display.rs       # Output formatting with colors
│
├── tests/
│   ├── integration/         # Integration tests
│   │   ├── init_test.rs
│   │   ├── add_test.rs
│   │   ├── push_test.rs
│   │   ├── pull_test.rs
│   │   ├── status_test.rs
│   │   ├── conflict_test.rs
│   │   └── workflow_test.rs # Complete end-to-end test
│   │
│   └── common/              # Test helpers
│       └── mod.rs           # Setup test projects, etc
│
└── examples/
    └── basic_workflow.md    # Usage examples
```

---

## 10. Acceptance Criteria

### Per Command:

**init:**
- ✅ Creates correct structure in `~/.local/git-shade/`
- ✅ Fails if not a git repo
- ✅ Fails if shade repo not cloned
- ✅ Detects files in shade and asks to pull
- ✅ Updates `config.toml` correctly

**add:**
- ✅ Adds to `.git/info/exclude` without duplicates
- ✅ Copies files preserving directory structure
- ✅ Fails if file doesn't exist
- ✅ Handles directories recursively
- ✅ Can add multiple files in one command

**push:**
- ✅ Copies all tracked files local → shade
- ✅ Creates commit with appropriate message
- ✅ Push successful to remote (if configured)
- ✅ Updates `last_push` timestamp
- ✅ Fails gracefully if no remote

**pull:**
- ✅ Runs `git pull` in shade repo first
- ✅ Detects conflicts correctly
- ✅ Aborts if conflicts exist (without `--force`)
- ✅ Shows clear resolution instructions
- ✅ Safe pull when only remote changed
- ✅ Skips files when only local changed
- ✅ `--force` overwrites without asking
- ✅ `--dry-run` shows plan without executing
- ✅ Updates `last_pull` timestamp
- ✅ Adds new files to `.git/info/exclude`

**status:**
- ✅ Shows all states (✓ ↑ ↓ ⚠ ? ←)
- ✅ Appropriate colors
- ✅ Readable timestamps
- ✅ Clear legend
- ✅ Git repo info
