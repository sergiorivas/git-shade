# Complete Implementation Plan - Step by Step

**For experienced Ruby/Go/JS/Elixir developer learning Rust**

---

## Week 1: Setup and Core Infrastructure

### Day 1: Project Setup and Understanding Rust Basics (3-4 hours)

#### Task 1.1: Create Project (30 min)

```bash
# Install Rust if you haven't
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Create project
cargo new git-shade
cd git-shade

# Test it works
cargo run
# Should print "Hello, world!"
```

**Language comparisons:**
- `cargo` = `bundler` (Ruby) + `go mod` (Go) + `npm` (JS) + `mix` (Elixir)
- `Cargo.toml` = `Gemfile` + `go.mod` + `package.json` + `mix.exs`
- `cargo run` compiles and runs (like Go, unlike Ruby/JS/Elixir which are interpreted/VM-based)

#### Task 1.2: Configure Cargo.toml (15 min)

```toml
[package]
name = "git-shade"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tempfile = "3.10"
```

**Understanding:**
- Like `Gemfile` (Ruby), `go.mod` (Go), `package.json` (JS), `mix.exs` (Elixir)
- `edition = "2021"` is the Rust language version (like Go versions: go1.21)
- `features = ["derive"]` enables optional functionality (like Ruby's gem extras)

#### Task 1.3: Create Directory Structure (15 min)

```bash
mkdir -p src/{commands,core,git,utils}
mkdir -p tests/{integration,common}
touch src/commands/mod.rs
touch src/core/mod.rs
touch src/git/mod.rs
touch src/utils/mod.rs
```

**Module system comparison:**

```rust
// Rust - explicit modules in mod.rs
pub mod commands;
pub mod core;
```

```ruby
# Ruby - implicit with autoload
module GitShade
  autoload :Commands, 'git_shade/commands'
end
```

```go
// Go - implicit by directory
package commands
```

```javascript
// JS - explicit exports
export { init } from './commands/init.js';
```

```elixir
# Elixir - explicit modules
defmodule GitShade.Commands do
end
```

#### Task 1.4: Hello CLI with Clap (2 hours)

Create `src/cli.rs`:

```rust
use clap::{Parser, Subcommand};

// #[derive(Parser)] is like:
// - Ruby's meta-programming (define_method)
// - Go's struct tags (json:"name")
// - JS decorators (@command)
// - Elixir's use macro (use Ecto.Schema)
#[derive(Parser)]
#[command(name = "git-shade")]
#[command(about = "Sync git-excluded files across machines")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Enum is like:
// - Ruby: class with subclasses (but type-safe)
// - Go: interface + type switch
// - Elixir: pattern matching with atoms
// - JS: discriminated unions (TypeScript)
#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(long)]
        name: Option<String>,  // Option = Maybe (Haskell), nullable (others)
    },
    Add {
        files: Vec<String>,  // Vec = Array/List/Slice
    },
    Push {
        #[arg(short, long)]
        message: Option<String>,
    },
    Pull {
        #[arg(long)]
        force: bool,
        #[arg(long)]
        dry_run: bool,
    },
    Status,
}
```

**Comparison table:**

| Rust | Ruby | Go | JavaScript | Elixir |
|------|------|-----|-----------|---------|
| `Option<T>` | `value or nil` | `*T` (pointer) | `value \| null` | `{:ok, value} \| nil` |
| `Vec<T>` | `Array` | `[]T` | `Array<T>` | `list` |
| `String` | `String` | `string` | `string` | `String.t()` |
| `bool` | `true/false` | `bool` | `boolean` | `boolean()` |

Update `src/main.rs`:

```rust
mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    // match is like:
    // - Ruby: case/when
    // - Go: switch (but exhaustive)
    // - JS: switch
    // - Elixir: case
    match cli.command {
        Commands::Init { name } => {
            println!("Init command with name: {:?}", name);
        },
        Commands::Add { files } => {
            println!("Add command with files: {:?}", files);
        },
        Commands::Push { message } => {
            println!("Push command with message: {:?}", message);
        },
        Commands::Pull { force, dry_run } => {
            println!("Pull command: force={}, dry_run={}", force, dry_run);
        },
        Commands::Status => {
            println!("Status command");
        },
    }
}
```

**Test it:**

```bash
cargo run -- --help
cargo run -- init --name myapp
cargo run -- add file1.txt file2.txt
```

**Key Rust concepts:**

1. **Ownership** (unique to Rust):
```rust
let s = String::from("hello");
let s2 = s;  // s is MOVED, can't use s anymore
```

```ruby
# Ruby - everything is a reference
s = "hello"
s2 = s  # Both point to same object
```

```go
// Go - value vs pointer semantics
s := "hello"
s2 := s  // Copy for strings
```

```elixir
# Elixir - immutable, always copies conceptually
s = "hello"
s2 = s  # Immutable, safe to share
```

2. **Borrowing** (unique to Rust):
```rust
let s = String::from("hello");
let len = calculate_length(&s);  // Borrow, don't move
println!("{}", s);  // Still valid!

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This prevents bugs at compile time that you'd catch at runtime in other languages.

---

### Day 2: Error Handling (3-4 hours)

#### Task 2.1: Create Error Types (2 hours)

Create `src/error.rs`:

```rust
use thiserror::Error;
use std::path::PathBuf;

// This is like defining custom exceptions/errors:
// Ruby: class ShadeError < StandardError
// Go: type ShadeError struct { ... }
// Elixir: defmodule ShadeError do ... end
// JS: class ShadeError extends Error
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
```

**Error handling comparison:**

```rust
// Rust - Result type (explicit)
fn read_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)?;  // ? = early return on error
    Ok(content)
}

// Usage
match read_file("config.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("Error: {}", e),
}
```

```ruby
# Ruby - exceptions (implicit)
def read_file(path)
  File.read(path)
rescue Errno::ENOENT => e
  raise ShadeError, "File not found: #{path}"
end

# Usage
begin
  content = read_file("config.txt")
  puts content
rescue ShadeError => e
  puts "Error: #{e.message}"
end
```

```go
// Go - error value (explicit)
func readFile(path string) (string, error) {
    content, err := os.ReadFile(path)
    if err != nil {
        return "", fmt.Errorf("failed to read: %w", err)
    }
    return string(content), nil
}

// Usage
content, err := readFile("config.txt")
if err != nil {
    log.Fatalf("Error: %v", err)
}
fmt.Println(content)
```

```elixir
# Elixir - pattern matching with tuples
def read_file(path) do
  case File.read(path) do
    {:ok, content} -> {:ok, content}
    {:error, reason} -> {:error, "Failed: #{reason}"}
  end
end

# Usage
case read_file("config.txt") do
  {:ok, content} -> IO.puts(content)
  {:error, msg} -> IO.puts("Error: #{msg}")
end
```

```javascript
// JavaScript - exceptions or Promises
async function readFile(path) {
    try {
        const content = await fs.readFile(path, 'utf8');
        return content;
    } catch (e) {
        throw new ShadeError(`File not found: ${path}`);
    }
}

// Usage
try {
    const content = await readFile('config.txt');
    console.log(content);
} catch (e) {
    console.error('Error:', e.message);
}
```

**Why Rust's approach is powerful:**
- Compiler forces you to handle errors (can't ignore `Result`)
- `?` operator makes it ergonomic (like Elixir's `with` or Go's `if err != nil`)
- Zero runtime overhead (unlike exceptions)

#### Task 2.2: Update main.rs to use Result (1 hour)

Update `src/main.rs`:

```rust
mod cli;
mod error;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { name } => {
            println!("Init: {:?}", name);
            Ok(())  // () is like nil/void/unit
        },
        Commands::Add { files } => {
            println!("Add: {:?}", files);
            Ok(())
        },
        Commands::Push { message } => {
            println!("Push: {:?}", message);
            Ok(())
        },
        Commands::Pull { force, dry_run } => {
            println!("Pull: force={}, dry_run={}", force, dry_run);
            Ok(())
        },
        Commands::Status => {
            println!("Status");
            Ok(())
        },
    }
}
```

**The `?` operator:**

```rust
// Without ?
fn do_thing() -> Result<String> {
    let file = match std::fs::read_to_string("file.txt") {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };
    let parsed = match parse(&file) {
        Ok(p) => p,
        Err(e) => return Err(e.into()),
    };
    Ok(parsed)
}

// With ?
fn do_thing() -> Result<String> {
    let file = std::fs::read_to_string("file.txt")?;
    let parsed = parse(&file)?;
    Ok(parsed)
}
```

Similar to:
- **Elixir**: `with` statement
- **Haskell**: `>>=` (bind operator)
- **Go**: `if err != nil { return err }`

#### Task 2.3: Write Basic Tests (1 hour)

Create `tests/common/mod.rs`:

```rust
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
```

Create `tests/integration/basic_test.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_works() {
    Command::cargo_bin("git-shade")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("git-shade"));
}
```

**Testing comparison:**

```rust
// Rust - built-in test framework
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_error() {
    let result = divide(10, 0);
    assert!(result.is_err());
}
```

```ruby
# Ruby - RSpec
describe "Math" do
  it "adds numbers" do
    expect(2 + 2).to eq(4)
  end

  it "handles errors" do
    expect { divide(10, 0) }.to raise_error(ZeroDivisionError)
  end
end
```

```go
// Go - testing package
func TestAddition(t *testing.T) {
    if 2 + 2 != 4 {
        t.Errorf("Expected 4, got %d", 2+2)
    }
}
```

```elixir
# Elixir - ExUnit
defmodule MathTest do
  use ExUnit.Case

  test "adds numbers" do
    assert 2 + 2 == 4
  end
end
```

**Run tests:**

```bash
cargo test
cargo test test_help_works  # Run specific test
cargo test -- --nocapture   # Show println! output
```

---

### Day 3: Paths and Configuration (4-5 hours)

#### Task 3.1: ShadePaths Struct (2 hours)

Create `src/core/paths.rs`:

```rust
use std::path::PathBuf;
use anyhow::{Context, Result};

// Struct is like:
// - Ruby: class with attr_accessor
// - Go: struct with fields
// - Elixir: defstruct
// - JS: class with properties
pub struct ShadePaths {
    pub root: PathBuf,
    pub config: PathBuf,
    pub metadata: PathBuf,
    pub projects: PathBuf,
}

// impl = implementation block (like Ruby's class methods)
impl ShadePaths {
    // Associated function (like Ruby's self.new or Go's NewShadePaths)
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        let root = home.join(".local/git-shade");

        Ok(Self {  // Self = ShadePaths (like @class in Ruby)
            config: root.join("config.toml"),
            metadata: root.join("metadata"),
            projects: root.join("projects"),
            root,
        })
    }

    // Instance method (&self = readonly access, like Ruby's regular method)
    pub fn ensure_structure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.metadata)
            .context("Failed to create metadata directory")?;
        Ok(())
    }

    // &self borrows the struct (doesn't consume it)
    pub fn project_metadata_dir(&self, project_name: &str) -> PathBuf {
        self.metadata.join(project_name)
    }

    pub fn project_shade_dir(&self, project_name: &str) -> PathBuf {
        self.projects.join(project_name)
    }

    pub fn shade_sync_file(&self, project_name: &str) -> PathBuf {
        self.project_metadata_dir(project_name).join(".shade-sync")
    }
}

#[cfg(test)]  // Only compiled for tests
mod tests {
    use super::*;

    #[test]
    fn test_paths_structure() {
        let paths = ShadePaths::new().unwrap();
        assert!(paths.root.ends_with(".local/git-shade"));
        assert!(paths.config.ends_with("config.toml"));
    }
}
```

**Ownership & Borrowing explained:**

```rust
// Ownership
let paths = ShadePaths::new().unwrap();
let other = paths;  // paths is MOVED, can't use anymore

// Borrowing (read-only)
let paths = ShadePaths::new().unwrap();
let dir = paths.project_metadata_dir("myapp");  // Borrows &self
println!("{:?}", paths);  // Still valid!

// Mutable borrowing
let mut data = vec![1, 2, 3];
data.push(4);  // Needs &mut self
```

**Language comparisons:**

```ruby
# Ruby - everything is a reference
class ShadePaths
  attr_reader :root, :config

  def initialize
    @root = File.join(Dir.home, ".local/git-shade")
    @config = File.join(@root, "config.toml")
  end

  def project_metadata_dir(project_name)
    File.join(@metadata, project_name)
  end
end
```

```go
// Go - explicit pointers
type ShadePaths struct {
    Root     string
    Config   string
    Metadata string
    Projects string
}

func NewShadePaths() (*ShadePaths, error) {
    home, err := os.UserHomeDir()
    if err != nil {
        return nil, err
    }

    root := filepath.Join(home, ".local/git-shade")
    return &ShadePaths{
        Root:     root,
        Config:   filepath.Join(root, "config.toml"),
        Metadata: filepath.Join(root, "metadata"),
        Projects: filepath.Join(root, "projects"),
    }, nil
}

func (p *ShadePaths) ProjectMetadataDir(projectName string) string {
    return filepath.Join(p.Metadata, projectName)
}
```

```elixir
# Elixir - immutable structs
defmodule ShadePaths do
  defstruct [:root, :config, :metadata, :projects]

  def new do
    home = System.user_home!()
    root = Path.join([home, ".local/git-shade"])

    %__MODULE__{
      root: root,
      config: Path.join(root, "config.toml"),
      metadata: Path.join(root, "metadata"),
      projects: Path.join(root, "projects")
    }
  end

  def project_metadata_dir(%__MODULE__{metadata: metadata}, project_name) do
    Path.join(metadata, project_name)
  end
end
```

Update `src/core/mod.rs`:

```rust
pub mod paths;

pub use paths::ShadePaths;  // Re-export for convenience
```

#### Task 3.2: Config Handling (2 hours)

Create `src/core/config.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

// Serialize/Deserialize are like:
// - Ruby: to_json/from_json (ActiveSupport)
// - Go: json tags
// - Elixir: Jason.encode!/decode!
// - JS: JSON.stringify/parse
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: String,
    #[serde(default)]  // If missing in TOML, use Vec::new()
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub local_path: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                version: "1.0".to_string(),
                projects: Vec::new(),
            });
        }

        let contents = std::fs::read_to_string(path)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, contents)
            .context("Failed to write config file")?;

        Ok(())
    }

    // &mut self = mutable borrow (can modify)
    pub fn add_project(&mut self, name: String, local_path: PathBuf) -> Result<()> {
        if self.projects.iter().any(|p| p.name == name) {
            anyhow::bail!("Project already exists: {}", name);
        }

        self.projects.push(Project { name, local_path });
        Ok(())
    }

    // Returns Option (like Ruby's nil, Go's nil, Elixir's nil)
    pub fn find_project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }
}
```

**Iterators comparison:**

```rust
// Rust - zero-cost abstractions
let found = self.projects
    .iter()                     // Iterator
    .find(|p| p.name == name)   // Filter
    .map(|p| &p.local_path);    // Transform
```

```ruby
# Ruby - Enumerable
found = projects.find { |p| p.name == name }
```

```go
// Go - manual loops
var found *Project
for _, p := range projects {
    if p.Name == name {
        found = &p
        break
    }
}
```

```javascript
// JavaScript - array methods
const found = projects.find(p => p.name === name);
```

```elixir
# Elixir - Enum module
found = Enum.find(projects, fn p -> p.name == name end)
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
# ... existing
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"
```

Add test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_save_and_load() {
        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.toml");

        let mut config = Config {
            version: "1.0".to_string(),
            projects: Vec::new(),
        };

        config.add_project(
            "myapp".to_string(),
            PathBuf::from("/home/user/projects/myapp")
        ).unwrap();

        config.save(&config_path).unwrap();

        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].name, "myapp");
    }
}
```

Update `src/core/mod.rs`:

```rust
pub mod paths;
pub mod config;

pub use paths::ShadePaths;
pub use config::{Config, Project};
```

---

### Day 4: Tracker and Basic Commands (4-5 hours)

#### Task 4.1: Tracker (Timestamps) (1.5 hours)

Create `src/core/tracker.rs`:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tracker {
    pub last_pull: Option<DateTime<Utc>>,
    pub last_push: Option<DateTime<Utc>>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            last_pull: None,
            last_push: None,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let contents = std::fs::read_to_string(path)?;
        let tracker: Tracker = toml::from_str(&contents)?;
        Ok(tracker)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn update_pull(&mut self) {
        self.last_pull = Some(Utc::now());
    }

    pub fn update_push(&mut self) {
        self.last_push = Some(Utc::now());
    }
}
```

**DateTime comparison:**

```rust
// Rust - chrono crate
use chrono::{DateTime, Utc};
let now: DateTime<Utc> = Utc::now();
```

```ruby
# Ruby - Time/DateTime
now = Time.now.utc
```

```go
// Go - time package
import "time"
now := time.Now().UTC()
```

```javascript
// JavaScript - Date
const now = new Date();
```

```elixir
# Elixir - DateTime
now = DateTime.utc_now()
```

Update `src/core/mod.rs`:

```rust
pub mod paths;
pub mod config;
pub mod tracker;

pub use paths::ShadePaths;
pub use config::{Config, Project};
pub use tracker::Tracker;
```

#### Task 4.2: Detect Current Project (1 hour)

Create `src/utils/project.rs`:

```rust
use std::path::{Path, PathBuf};
use std::env;
use anyhow::{Context, Result};
use crate::error::ShadeError;

pub fn detect_project_name(name_override: Option<String>) -> Result<String> {
    if let Some(name) = name_override {
        return Ok(name);
    }

    let current_dir = env::current_dir()?;

    // Get directory name
    let name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Could not determine project name"))?;

    Ok(name)
}

pub fn verify_git_repo() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let git_dir = current_dir.join(".git");

    if !git_dir.exists() {
        return Err(ShadeError::NotGitRepo.into());
    }

    Ok(current_dir)
}
```

**Option and Result chaining:**

```rust
// Rust - monadic chaining
let name = current_dir
    .file_name()              // Option<&OsStr>
    .and_then(|n| n.to_str()) // Option<&str>
    .map(|s| s.to_string())   // Option<String>
    .ok_or_else(|| error)?;   // Result<String>
```

```ruby
# Ruby - safe navigation
name = current_dir&.basename&.to_s || raise("Error")
```

```elixir
# Elixir - with statement
with {:ok, basename} <- get_basename(current_dir),
     name <- to_string(basename) do
  {:ok, name}
else
  _ -> {:error, "Could not determine name"}
end
```

```go
// Go - explicit checks
basename := filepath.Base(currentDir)
if basename == "" {
    return "", errors.New("could not determine name")
}
```

Create `src/utils/mod.rs`:

```rust
pub mod project;

pub use project::{detect_project_name, verify_git_repo};
```

#### Task 4.3: Implement `init` Command (2 hours)

Create `src/commands/init.rs`:

```rust
use crate::core::{ShadePaths, Config, Tracker};
use crate::utils::{detect_project_name, verify_git_repo};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use dialoguer::Confirm;
use std::fs;
use walkdir::WalkDir;

pub fn run(name_override: Option<String>) -> Result<()> {
    // 1. Verify it's a git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project name
    let project_name = detect_project_name(name_override)?;

    // 3. Setup paths
    let paths = ShadePaths::new()?;

    // 4. Verify shade repo exists
    if !paths.projects.join(".git").exists() {
        return Err(ShadeError::ShadeRepoNotFound);
    }

    // 5. Check if already initialized
    let mut config = Config::load(&paths.config)?;
    if config.find_project(&project_name).is_some() {
        return Err(ShadeError::AlreadyInitialized(project_name));
    }

    // 6. Create metadata directory
    paths.ensure_structure()?;
    let project_metadata_dir = paths.project_metadata_dir(&project_name);
    fs::create_dir_all(&project_metadata_dir)?;

    // 7. Create tracker file
    let tracker = Tracker::new();
    tracker.save(&paths.shade_sync_file(&project_name))?;

    // 8. Create project directory in shade
    let project_shade_dir = paths.project_shade_dir(&project_name);
    fs::create_dir_all(&project_shade_dir)?;

    // 9. Add to config
    config.add_project(project_name.clone(), project_path)?;
    config.save(&paths.config)?;

    // 10. Print success
    println!("{} Initialized git-shade for project: {}",
        "✓".green().bold(),
        project_name.bold()
    );
    println!("  Config: {}", paths.config.display());
    println!("  Metadata: {}", project_metadata_dir.display());
    println!("  Shade dir: {}", project_shade_dir.display());
    println!();

    // 11. Check if shade has files
    let existing_files = list_shade_files(&project_shade_dir)?;

    if !existing_files.is_empty() {
        println!("Found {} files in shade:", existing_files.len());
        for file in &existing_files {
            println!("  - {}", file.display());
        }
        println!();

        // Ask to pull
        let should_pull = Confirm::new()
            .with_prompt("Pull these files now?")
            .default(true)
            .interact()?;

        if should_pull {
            pull_files(&existing_files, &project_shade_dir, &project_path)?;
            add_to_exclude(&project_path, &existing_files, &project_shade_dir)?;
            println!();
            println!("{} Done!", "✓".green().bold());
        } else {
            println!("Skipped. Pull manually with: {}", "git-shade pull".bold());
        }
    }

    Ok(())
}

fn list_shade_files(shade_dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(shade_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Ok(rel) = entry.path().strip_prefix(shade_dir) {
                files.push(rel.to_path_buf());
            }
        }
    }

    Ok(files)
}

fn pull_files(
    files: &[std::path::PathBuf],
    shade_dir: &std::path::Path,
    project_dir: &std::path::Path,
) -> Result<()> {
    use crate::utils::copy_file_preserve_structure;

    println!("Pulling files...");
    for file in files {
        let src = shade_dir.join(file);
        copy_file_preserve_structure(&src, shade_dir, project_dir)?;
        println!("  {} {}", "✓".green(), file.display());
    }

    Ok(())
}

fn add_to_exclude(
    project_dir: &std::path::Path,
    files: &[std::path::PathBuf],
    shade_dir: &std::path::Path,
) -> Result<()> {
    use crate::git::add_to_exclude as git_add_to_exclude;

    let patterns: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    git_add_to_exclude(project_dir, &patterns)?;
    Ok(())
}
```

Add to `Cargo.toml`:

```toml
[dependencies]
# ... existing
colored = "2.1"
dialoguer = "0.11"
walkdir = "2.5"
```

Create `src/commands/mod.rs`:

```rust
pub mod init;
```

Update `src/main.rs`:

```rust
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
        Commands::Add { files } => {
            println!("Add: {:?}", files);
            Ok(())
        },
        Commands::Push { message } => {
            println!("Push: {:?}", message);
            Ok(())
        },
        Commands::Pull { force, dry_run } => {
            println!("Pull: force={}, dry_run={}", force, dry_run);
            Ok(())
        },
        Commands::Status => {
            println!("Status");
            Ok(())
        },
    }
}
```

---

## Week 2: Core Commands (add, push, pull)

### Day 5: Command `add` (4-5 hours)

#### Task 5.1: File Operations Utilities (2 hours)

Create `src/utils/fs.rs`:

```rust
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Result};
use walkdir::WalkDir;

/// Copy a file preserving directory structure
pub fn copy_file_preserve_structure(
    src: &Path,
    src_base: &Path,
    dst_base: &Path,
) -> Result<PathBuf> {
    // Get relative path
    let rel_path = src.strip_prefix(src_base)
        .context("Failed to get relative path")?;

    let dst = dst_base.join(rel_path);

    // Create parent directory
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy file
    fs::copy(src, &dst)?;

    Ok(dst)
}

/// Copy directory recursively
pub fn copy_dir_recursive(
    src: &Path,
    src_base: &Path,
    dst_base: &Path,
) -> Result<Vec<PathBuf>> {
    let mut copied = Vec::new();

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let dst = copy_file_preserve_structure(path, src_base, dst_base)?;
            copied.push(dst);
        }
    }

    Ok(copied)
}
```

**File operations comparison:**

```rust
// Rust - explicit error handling
let content = fs::read_to_string("file.txt")?;
fs::write("output.txt", content)?;
```

```ruby
# Ruby - exceptions
content = File.read("file.txt")
File.write("output.txt", content)
```

```go
// Go - explicit error checks
content, err := os.ReadFile("file.txt")
if err != nil {
    return err
}
err = os.WriteFile("output.txt", content, 0644)
```

```javascript
// JavaScript - Promises
const content = await fs.promises.readFile('file.txt', 'utf8');
await fs.promises.writeFile('output.txt', content);
```

```elixir
# Elixir - pattern matching
{:ok, content} = File.read("file.txt")
:ok = File.write("output.txt", content)
```

Update `src/utils/mod.rs`:

```rust
pub mod project;
pub mod fs;

pub use project::{detect_project_name, verify_git_repo};
pub use fs::{copy_file_preserve_structure, copy_dir_recursive};
```

#### Task 5.2: Git Exclude Handling (1.5 hours)

Create `src/git/exclude.rs`:

```rust
use std::path::Path;
use std::fs;
use anyhow::{Context, Result};

pub fn add_to_exclude(repo_path: &Path, patterns: &[String]) -> Result<()> {
    let exclude_file = repo_path.join(".git/info/exclude");

    // Read existing
    let existing = if exclude_file.exists() {
        fs::read_to_string(&exclude_file)?
    } else {
        String::new()
    };

    let mut lines: Vec<String> = existing.lines().map(|s| s.to_string()).collect();

    // Add new patterns
    let mut added = Vec::new();
    for pattern in patterns {
        if !lines.contains(pattern) {
            lines.push(pattern.clone());
            added.push(pattern.clone());
        }
    }

    if added.is_empty() {
        return Ok(());
    }

    // Write back
    let content = lines.join("\n") + "\n";

    if let Some(parent) = exclude_file.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&exclude_file, content)?;

    println!("✓ Added to .git/info/exclude:");
    for pattern in &added {
        println!("  - {}", pattern);
    }

    Ok(())
}

pub fn read_exclude(repo_path: &Path) -> Result<Vec<String>> {
    let exclude_file = repo_path.join(".git/info/exclude");

    if !exclude_file.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&exclude_file)?;

    let patterns: Vec<String> = content
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .map(|s| s.to_string())
        .collect();

    Ok(patterns)
}
```

**Functional programming comparison:**

```rust
// Rust - iterator chains (zero-cost)
let patterns: Vec<String> = content
    .lines()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| s.to_string())
    .collect();
```

```ruby
# Ruby - Enumerable
patterns = content
  .lines
  .map(&:strip)
  .reject(&:empty?)
```

```go
// Go - manual loops (most efficient)
var patterns []string
for _, line := range strings.Split(content, "\n") {
    trimmed := strings.TrimSpace(line)
    if trimmed != "" && !strings.HasPrefix(trimmed, "#") {
        patterns = append(patterns, trimmed)
    }
}
```

```javascript
// JavaScript - array methods
const patterns = content
    .split('\n')
    .map(s => s.trim())
    .filter(s => s && !s.startsWith('#'));
```

```elixir
# Elixir - Enum pipeline
patterns =
  content
  |> String.split("\n")
  |> Enum.map(&String.trim/1)
  |> Enum.reject(&(&1 == "" or String.starts_with?(&1, "#")))
```

Create `src/git/mod.rs`:

```rust
pub mod exclude;

pub use exclude::{add_to_exclude, read_exclude};
```

#### Task 5.3: Implement `add` Command (1.5 hours)

Create `src/commands/add.rs`:

```rust
use std::path::Path;
use std::env;
use crate::core::{ShadePaths, Config};
use crate::utils::{detect_project_name, verify_git_repo, copy_file_preserve_structure, copy_dir_recursive};
use crate::git::add_to_exclude;
use crate::error::{Result, ShadeError};
use colored::Colorize;

pub fn run(files: Vec<String>) -> Result<()> {
    // 1. Verify git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project
    let project_name = detect_project_name(None)?;

    // 3. Verify initialized
    let paths = ShadePaths::new()?;
    let config = Config::load(&paths.config)?;
    config.find_project(&project_name)
        .ok_or(ShadeError::NotInitialized)?;

    // 4. Verify files exist
    let mut patterns = Vec::new();
    let mut files_to_copy = Vec::new();

    for file_str in &files {
        let file_path = Path::new(file_str);
        let abs_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            env::current_dir()?.join(file_path)
        };

        if !abs_path.exists() {
            return Err(ShadeError::FileNotFound(abs_path));
        }

        patterns.push(file_str.clone());
        files_to_copy.push(abs_path);
    }

    // 5. Add to .git/info/exclude
    add_to_exclude(&project_path, &patterns)?;

    // 6. Copy files to shade
    let shade_dir = paths.project_shade_dir(&project_name);

    println!();
    println!("✓ Copied to {}:", shade_dir.display());

    for file_path in &files_to_copy {
        if file_path.is_file() {
            let dst = copy_file_preserve_structure(file_path, &project_path, &shade_dir)?;
            println!("  - {}", dst.strip_prefix(&shade_dir).unwrap().display());
        } else if file_path.is_dir() {
            let dsts = copy_dir_recursive(file_path, &project_path, &shade_dir)?;
            for dst in &dsts {
                println!("  - {}", dst.strip_prefix(&shade_dir).unwrap().display());
            }
        }
    }

    println!();
    println!("Ready to push with: {}", "git-shade push".bold());

    Ok(())
}
```

**Pattern matching on types:**

```rust
// Rust - explicit type checking
if file_path.is_file() {
    // handle file
} else if file_path.is_dir() {
    // handle directory
}
```

```ruby
# Ruby - respond_to? or case
case
when file_path.file?
  # handle file
when file_path.directory?
  # handle directory
end
```

```go
// Go - FileInfo Mode()
info, err := os.Stat(filePath)
if err != nil {
    return err
}

if info.Mode().IsRegular() {
    // handle file
} else if info.IsDir() {
    // handle directory
}
```

```elixir
# Elixir - File.stat
case File.stat(file_path) do
  {:ok, %File.Stat{type: :regular}} ->
    # handle file
  {:ok, %File.Stat{type: :directory}} ->
    # handle directory
  {:error, reason} ->
    {:error, reason}
end
```

Update `src/commands/mod.rs`:

```rust
pub mod init;
pub mod add;
```

Update `src/main.rs`:

```rust
// In match
Commands::Add { files } => commands::add::run(files),
```

---

### Day 6-7: Command `push` (6-8 hours)

#### Task 6.1: Git Wrapper (3 hours)

Create `src/git/repo.rs`:

```rust
use std::path::Path;
use std::process::{Command, Output};
use anyhow::{Context, Result};
use crate::error::ShadeError;

pub struct GitRepo {
    path: std::path::PathBuf,
}

impl GitRepo {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.path.join(".git").exists()
    }

    pub fn add_all(&self) -> Result<()> {
        self.run_command(&["add", "."])?;
        Ok(())
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        self.run_command(&["commit", "-m", message])?;
        Ok(())
    }

    pub fn push(&self, remote: &str, branch: &str) -> Result<()> {
        self.run_command(&["push", remote, branch])?;
        Ok(())
    }

    pub fn pull(&self, remote: &str, branch: &str) -> Result<()> {
        self.run_command(&["pull", remote, branch])?;
        Ok(())
    }

    pub fn get_remote(&self, name: &str) -> Result<Option<String>> {
        let output = self.run_command(&["remote", "get-url", name]);

        match output {
            Ok(out) => {
                let url = String::from_utf8_lossy(&out.stdout)
                    .trim()
                    .to_string();
                Ok(Some(url))
            },
            Err(_) => Ok(None),
        }
    }

    pub fn has_changes(&self) -> Result<bool> {
        let output = self.run_command(&["status", "--porcelain"])?;
        Ok(!output.stdout.is_empty())
    }

    fn run_command(&self, args: &[&str]) -> Result<Output> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.path)
            .output()
            .context("Failed to execute git command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ShadeError::GitError(stderr.to_string()).into());
        }

        Ok(output)
    }
}
```

**Process execution comparison:**

```rust
// Rust - Command builder
let output = Command::new("git")
    .args(&["status", "--porcelain"])
    .current_dir(&path)
    .output()?;

if !output.status.success() {
    return Err(error);
}
```

```ruby
# Ruby - backticks or system
result = `git status --porcelain`
raise "Command failed" unless $?.success?

# Or Open3
require 'open3'
stdout, stderr, status = Open3.capture3("git", "status", "--porcelain", chdir: path)
raise "Failed: #{stderr}" unless status.success?
```

```go
// Go - exec.Command
cmd := exec.Command("git", "status", "--porcelain")
cmd.Dir = path
output, err := cmd.CombinedOutput()
if err != nil {
    return fmt.Errorf("git failed: %w", err)
}
```

```javascript
// JavaScript - child_process
const { execSync } = require('child_process');
try {
    const output = execSync('git status --porcelain', {
        cwd: path,
        encoding: 'utf8'
    });
} catch (e) {
    throw new Error(`Git failed: ${e.message}`);
}
```

```elixir
# Elixir - System.cmd
case System.cmd("git", ["status", "--porcelain"], cd: path) do
  {output, 0} -> {:ok, output}
  {output, _} -> {:error, output}
end
```

Update `src/git/mod.rs`:

```rust
pub mod exclude;
pub mod repo;

pub use exclude::{add_to_exclude, read_exclude};
pub use repo::GitRepo;
```

#### Task 6.2: Implement `push` Command (3 hours)

Create `src/commands/push.rs`:

```rust
use std::env;
use crate::core::{ShadePaths, Config, Tracker};
use crate::utils::{detect_project_name, verify_git_repo, copy_file_preserve_structure};
use crate::git::{read_exclude, GitRepo};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use walkdir::WalkDir;

pub fn run(message: Option<String>) -> Result<()> {
    // 1. Verify git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project
    let project_name = detect_project_name(None)?;

    // 3. Verify initialized
    let paths = ShadePaths::new()?;
    let config = Config::load(&paths.config)?;
    config.find_project(&project_name)
        .ok_or(ShadeError::NotInitialized)?;

    // 4. Read tracked files from exclude
    let patterns = read_exclude(&project_path)?;
    if patterns.is_empty() {
        return Err(ShadeError::NoFilesTracked);
    }

    // 5. Copy files to shade
    let shade_dir = paths.project_shade_dir(&project_name);

    println!("Copying files to shade...");
    let mut copied_count = 0;

    for pattern in &patterns {
        let file_path = project_path.join(pattern);

        if file_path.is_file() {
            copy_file_preserve_structure(&file_path, &project_path, &shade_dir)?;
            println!("  {} {}", "✓".green(), pattern);
            copied_count += 1;
        } else if file_path.is_dir() {
            for entry in WalkDir::new(&file_path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    copy_file_preserve_structure(entry.path(), &project_path, &shade_dir)?;
                    if let Ok(rel) = entry.path().strip_prefix(&project_path) {
                        println!("  {} {}", "✓".green(), rel.display());
                        copied_count += 1;
                    }
                }
            }
        }
    }

    if copied_count == 0 {
        println!("No files to sync.");
        return Ok(());
    }

    println!();

    // 6. Git operations
    println!("Git operations in {}...", paths.projects.display());

    let git = GitRepo::new(&paths.projects);

    // Add only this project's files
    let project_pattern = format!("{}/", project_name);
    git.run_command(&["add", &project_pattern])?;

    // Create commit message
    let hostname = env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    let commit_msg = if let Some(msg) = message {
        format!("[{}] {}", project_name, msg)
    } else {
        format!("[{}] Update from {}", project_name, hostname)
    };

    // Commit
    git.commit(&commit_msg)?;
    println!("  {} Committed: {}", "✓".green(), commit_msg);

    // Push if remote exists
    if let Some(remote_url) = git.get_remote("origin")? {
        git.push("origin", "main")?;
        println!("  {} Pushed to origin/main", "✓".green());
    } else {
        println!();
        println!("{} No remote configured. Changes saved locally only.", "⚠".yellow());
        println!("  To sync across machines, add a remote:");
        println!("    cd {}", paths.projects.display());
        println!("    git remote add origin <url>");
    }

    // 7. Update tracker
    let mut tracker = Tracker::load(&paths.shade_sync_file(&project_name))?;
    tracker.update_push();
    tracker.save(&paths.shade_sync_file(&project_name))?;

    println!();
    println!("Updated last_push: {}",
        tracker.last_push.unwrap().format("%Y-%m-%d %H:%M:%S UTC"));

    Ok(())
}
```

**String formatting comparison:**

```rust
// Rust - format! macro
let msg = format!("[{}] Update from {}", project_name, hostname);
```

```ruby
# Ruby - string interpolation
msg = "[#{project_name}] Update from #{hostname}"
```

```go
// Go - fmt.Sprintf
msg := fmt.Sprintf("[%s] Update from %s", projectName, hostname)
```

```javascript
// JavaScript - template literals
const msg = `[${projectName}] Update from ${hostname}`;
```

```elixir
# Elixir - string interpolation
msg = "[#{project_name}] Update from #{hostname}"
```

Add method to `src/git/repo.rs`:

```rust
impl GitRepo {
    // ... existing methods

    pub fn run_command(&self, args: &[&str]) -> Result<Output> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.path)
            .output()
            .context("Failed to execute git command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ShadeError::GitError(stderr.to_string()).into());
        }

        Ok(output)
    }
}
```

Update `src/commands/mod.rs`:

```rust
pub mod init;
pub mod add;
pub mod push;
```

Update `src/main.rs`:

```rust
Commands::Push { message } => commands::push::run(message),
```

---

### Day 8-10: Command `pull` with Conflict Detection (8-10 hours)

#### Task 8.1: Conflict Detection Logic (3 hours)

Create `src/core/conflict.rs`:

```rust
use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum SyncState {
    InSync,
    LocalAhead,
    RemoteAhead,
    Conflict,
    LocalOnly,
    RemoteOnly,
    NotTracked,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub modified: DateTime<Utc>,
    pub size: u64,
}

impl FileMetadata {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let modified_dt = DateTime::from(modified);

        Ok(Self {
            path: path.to_path_buf(),
            modified: modified_dt,
            size: metadata.len(),
        })
    }
}

pub fn detect_sync_state(
    local_file: Option<FileMetadata>,
    remote_file: Option<FileMetadata>,
    last_pull: Option<DateTime<Utc>>,
) -> SyncState {
    match (local_file, remote_file, last_pull) {
        (None, None, _) => SyncState::NotTracked,
        (Some(_), None, _) => SyncState::LocalOnly,
        (None, Some(_), _) => SyncState::RemoteOnly,

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

        (Some(local), Some(remote), None) => {
            if local.modified == remote.modified && local.size == remote.size {
                SyncState::InSync
            } else {
                SyncState::RemoteAhead
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_detect_in_sync() {
        let now = Utc::now();
        let local = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now,
            size: 100,
        };
        let remote = local.clone();
        let last_pull = now - Duration::hours(1);

        let state = detect_sync_state(Some(local), Some(remote), Some(last_pull));
        assert_eq!(state, SyncState::InSync);
    }

    #[test]
    fn test_detect_conflict() {
        let now = Utc::now();
        let last_pull = now - Duration::hours(2);

        let local = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now - Duration::hours(1),
            size: 100,
        };
        let remote = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now - Duration::minutes(30),
            size: 120,
        };

        let state = detect_sync_state(Some(local), Some(remote), Some(last_pull));
        assert_eq!(state, SyncState::Conflict);
    }
}
```

**Pattern matching power:**

```rust
// Rust - exhaustive pattern matching
match (local_file, remote_file, last_pull) {
    (None, None, _) => SyncState::NotTracked,
    (Some(_), None, _) => SyncState::LocalOnly,
    (None, Some(_), _) => SyncState::RemoteOnly,
    (Some(local), Some(remote), Some(last_pull)) => {
        // Complex logic
    },
    (Some(local), Some(remote), None) => {
        // First time
    },
}
```

```elixir
# Elixir - similar pattern matching
case {local_file, remote_file, last_pull} do
  {nil, nil, _} -> :not_tracked
  {%File{}, nil, _} -> :local_only
  {nil, %File{}, _} -> :remote_only
  {%File{} = local, %File{} = remote, %DateTime{} = last_pull} ->
    # Complex logic
  {%File{} = local, %File{} = remote, nil} ->
    # First time
end
```

```go
// Go - type switches (less powerful)
switch {
case localFile == nil && remoteFile == nil:
    return NotTracked
case localFile != nil && remoteFile == nil:
    return LocalOnly
case localFile == nil && remoteFile != nil:
    return RemoteOnly
// ... more cases
}
```

```ruby
# Ruby - case/when (less type-safe)
case
when local_file.nil? && remote_file.nil?
  :not_tracked
when !local_file.nil? && remote_file.nil?
  :local_only
when local_file.nil? && !remote_file.nil?
  :remote_only
# ... more cases
end
```

Update `src/core/mod.rs`:

```rust
pub mod paths;
pub mod config;
pub mod tracker;
pub mod conflict;

pub use paths::ShadePaths;
pub use config::{Config, Project};
pub use tracker::Tracker;
pub use conflict::{SyncState, FileMetadata, detect_sync_state};
```

#### Task 8.2: Implement `pull` Command (5 hours)

Create `src/commands/pull.rs`:

```rust
use std::path::PathBuf;
use crate::core::{ShadePaths, Config, Tracker, SyncState, FileMetadata, detect_sync_state};
use crate::utils::{detect_project_name, verify_git_repo, copy_file_preserve_structure};
use crate::git::{read_exclude, GitRepo, add_to_exclude};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use walkdir::WalkDir;

pub fn run(force: bool, dry_run: bool) -> Result<()> {
    // 1. Verify git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project
    let project_name = detect_project_name(None)?;

    // 3. Verify initialized
    let paths = ShadePaths::new()?;
    let config = Config::load(&paths.config)?;
    config.find_project(&project_name)
        .ok_or(ShadeError::NotInitialized)?;

    // 4. Git pull in shade repo
    println!("Pulling from shade repo...");
    let git = GitRepo::new(&paths.projects);

    if !dry_run {
        git.pull("origin", "main")?;
        println!("  {} Git pull successful", "✓".green());
    } else {
        println!("  {} Git pull successful (dry-run)", "✓".green());
    }

    // 5. Load tracker
    let tracker = Tracker::load(&paths.shade_sync_file(&project_name))?;
    let last_pull = tracker.last_pull;

    // 6. Analyze files
    let shade_dir = paths.project_shade_dir(&project_name);
    let analysis = analyze_files(&project_path, &shade_dir, last_pull)?;

    // 7. Check for conflicts
    if !force && !analysis.conflicts.is_empty() {
        print_conflicts(&analysis.conflicts)?;
        return Err(ShadeError::ConflictDetected {
            files: analysis.conflicts.iter()
                .map(|f| f.path.to_string_lossy().to_string())
                .collect(),
        });
    }

    // 8. Sync files
    println!();
    println!("Checking for conflicts...");
    if analysis.conflicts.is_empty() {
        println!("  No conflicts detected");
    } else if force {
        println!("  {} Force mode: overwriting all local files", "⚠".yellow());
    }

    println!();
    println!("Syncing files...");

    let mut synced_count = 0;
    let mut skipped_count = 0;

    for item in &analysis.to_sync {
        match item.state {
            SyncState::RemoteAhead | SyncState::RemoteOnly => {
                if !dry_run {
                    let src = shade_dir.join(&item.path);
                    copy_file_preserve_structure(&src, &shade_dir, &project_path)?;
                }
                println!("  {} {} ({})",
                    "↓".blue(),
                    item.path.display(),
                    if dry_run { "would copy" } else { "copied" }
                );
                synced_count += 1;
            },
            SyncState::Conflict if force => {
                if !dry_run {
                    let src = shade_dir.join(&item.path);
                    copy_file_preserve_structure(&src, &shade_dir, &project_path)?;
                }
                println!("  {} {} (overwritten)", "✓".green(), item.path.display());
                synced_count += 1;
            },
            SyncState::InSync => {
                println!("  {} {} (in sync)", "-".white(), item.path.display());
                skipped_count += 1;
            },
            SyncState::LocalAhead => {
                println!("  {} {} (local newer, skipped)", "↑".yellow(), item.path.display());
                skipped_count += 1;
            },
            _ => {},
        }
    }

    // 9. Update exclude for new files
    if !dry_run {
        let new_patterns: Vec<String> = analysis.to_sync.iter()
            .filter(|item| item.state == SyncState::RemoteOnly)
            .map(|item| item.path.to_string_lossy().to_string())
            .collect();

        if !new_patterns.is_empty() {
            add_to_exclude(&project_path, &new_patterns)?;
        }
    }

    // 10. Update tracker
    if !dry_run {
        let mut tracker = tracker;
        tracker.update_pull();
        tracker.save(&paths.shade_sync_file(&project_name))?;

        println!();
        println!("Updated last_pull: {}",
            tracker.last_pull.unwrap().format("%Y-%m-%d %H:%M:%S UTC"));
    }

    println!();
    if dry_run {
        println!("{} Dry-run completed (no changes made)", "✓".green());
    } else if force {
        println!("{} Pull completed (forced)", "✓".green());
    } else {
        println!("{} Pull completed successfully", "✓".green());
    }

    Ok(())
}

struct FileAnalysis {
    path: PathBuf,
    state: SyncState,
}

struct Analysis {
    to_sync: Vec<FileAnalysis>,
    conflicts: Vec<FileAnalysis>,
}

fn analyze_files(
    project_path: &std::path::Path,
    shade_dir: &std::path::Path,
    last_pull: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Analysis> {
    let mut to_sync = Vec::new();
    let mut conflicts = Vec::new();

    // Get all files in shade
    for entry in WalkDir::new(shade_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let rel_path = entry.path().strip_prefix(shade_dir)?;
        let local_path = project_path.join(rel_path);
        let remote_path = entry.path();

        let local_meta = if local_path.exists() {
            Some(FileMetadata::from_path(&local_path)?)
        } else {
            None
        };

        let remote_meta = Some(FileMetadata::from_path(remote_path)?);

        let state = detect_sync_state(local_meta, remote_meta, last_pull);

        let analysis = FileAnalysis {
            path: rel_path.to_path_buf(),
            state: state.clone(),
        };

        if state == SyncState::Conflict {
            conflicts.push(analysis.clone());
        }

        to_sync.push(analysis);
    }

    Ok(Analysis { to_sync, conflicts })
}

fn print_conflicts(conflicts: &[FileAnalysis]) -> Result<()> {
    println!();
    println!("{}", "⚠ CONFLICTS DETECTED".red().bold());
    println!();
    println!("The following files were modified both locally and remotely since last pull:");
    println!();

    for conflict in conflicts {
        println!("  {} {}", "⚠".red(), conflict.path.display());
    }

    println!();
    println!("Manual resolution required:");
    println!("  1. Go to the shade directory and review remote versions");
    println!("  2. Choose which version to keep, OR manually merge");
    println!("  3. Copy resolved files to your project");
    println!("  4. OR use {} to overwrite local with remote",
        "'git-shade pull --force'".bold());
    println!();
    println!("Aborted. No files were modified.");

    Ok(())
}
```

**Vec/slice operations:**

```rust
// Rust - working with Vec
let conflicts: Vec<FileAnalysis> = analysis.to_sync
    .iter()
    .filter(|item| item.state == SyncState::Conflict)
    .cloned()
    .collect();

// Borrowing vs owned
let borrowed: &[FileAnalysis] = &conflicts;  // Slice (view)
let owned: Vec<FileAnalysis> = conflicts.clone();  // Copy
```

```ruby
# Ruby - arrays
conflicts = analysis.to_sync.select { |item|
  item.state == :conflict
}
```

```go
// Go - slices
var conflicts []FileAnalysis
for _, item := range analysis.ToSync {
    if item.State == Conflict {
        conflicts = append(conflicts, item)
    }
}
```

```elixir
# Elixir - lists
conflicts = Enum.filter(analysis.to_sync, fn item ->
  item.state == :conflict
end)
```

Update `src/commands/mod.rs`:

```rust
pub mod init;
pub mod add;
pub mod push;
pub mod pull;
```

Update `src/main.rs`:

```rust
Commands::Pull { force, dry_run } => commands::pull::run(force, dry_run),
```

---

## Week 3: Status Command, Testing, and Polish

### Day 11-12: Command `status` (4-6 hours)

#### Task 11.1: Implement `status` Command (4 hours)

Create `src/commands/status.rs`:

```rust
use crate::core::{ShadePaths, Config, Tracker, SyncState, FileMetadata, detect_sync_state};
use crate::utils::{detect_project_name, verify_git_repo};
use crate::git::{read_exclude, GitRepo};
use crate::error::{Result, ShadeError};
use colored::Colorize;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn run() -> Result<()> {
    // 1. Verify git repo
    let project_path = verify_git_repo()?;

    // 2. Detect project
    let project_name = detect_project_name(None)?;

    // 3. Verify initialized
    let paths = ShadePaths::new()?;
    let config = Config::load(&paths.config)?;
    config.find_project(&project_name)
        .ok_or(ShadeError::NotInitialized)?;

    // 4. Load tracker
    let tracker = Tracker::load(&paths.shade_sync_file(&project_name))?;

    // 5. Get tracked files
    let patterns = read_exclude(&project_path)?;

    if patterns.is_empty() {
        println!("Project: {}", project_name.bold());
        println!("No files tracked yet.");
        println!();
        println!("Add files with: {}", "git-shade add <files>".bold());
        return Ok(());
    }

    // 6. Header
    println!("Project: {}", project_name.bold());
    println!("Local:      {}", project_path.display());
    let shade_dir = paths.project_shade_dir(&project_name);
    println!("Shade:      {}", shade_dir.display());

    if let Some(last_pull) = tracker.last_pull {
        println!("Last pull:  {}", last_pull.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("Last pull:  {}", "never".dimmed());
    }

    if let Some(last_push) = tracker.last_push {
        println!("Last push:  {}", last_push.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("Last push:  {}", "never".dimmed());
    }

    println!();

    // 7. Analyze files
    let file_states = analyze_all_files(&project_path, &shade_dir, tracker.last_pull)?;

    // 8. Display files
    println!("Files:");

    for (path, state) in &file_states {
        let (symbol, color, description) = match state {
            SyncState::InSync => ("✓", "green", "in sync"),
            SyncState::LocalAhead => ("↑", "yellow", "local ahead - modified locally, ready to push"),
            SyncState::RemoteAhead => ("↓", "blue", "remote ahead - modified in shade, safe to pull"),
            SyncState::Conflict => ("⚠", "red", "conflict - modified both locally and remotely"),
            SyncState::LocalOnly => ("?", "white", "local only, not in shade repo"),
            SyncState::RemoteOnly => ("←", "white", "remote only, deleted locally"),
            SyncState::NotTracked => continue,
        };

        let colored_symbol = match color {
            "green" => symbol.green(),
            "yellow" => symbol.yellow(),
            "blue" => symbol.blue(),
            "red" => symbol.red(),
            _ => symbol.white(),
        };

        println!("  {} {} ({})", colored_symbol, path.display(), description);
    }

    // 9. Legend
    println!();
    println!("Legend:");
    println!("  {} In sync           Both files are identical", "✓".green());
    println!("  {} Local ahead       Modified locally, needs push", "↑".yellow());
    println!("  {} Remote ahead      Modified in shade, safe to pull", "↓".blue());
    println!("  {} Conflict          Modified in both places, manual resolution needed", "⚠".red());
    println!("  {} Local only        File exists locally but not in shade", "?".white());
    println!("  {} Remote only       File exists in shade but not locally", "←".white());

    // 10. Git remote info
    println!();
    let git = GitRepo::new(&paths.projects);
    if let Some(remote_url) = git.get_remote("origin")? {
        println!("Git remote: {}", remote_url);
    } else {
        println!("Git remote: {} - changes are local only", "(none)".dimmed());
        println!("  Add remote with:");
        println!("    cd {}", paths.projects.display());
        println!("    git remote add origin <url>");
    }

    // Git status
    if git.has_changes()? {
        println!("Git status: {} (uncommitted changes in shade repo)", "⚠".yellow());
    } else {
        println!("Git status: Clean (no uncommitted changes)");
    }

    Ok(())
}

fn analyze_all_files(
    project_path: &std::path::Path,
    shade_dir: &std::path::Path,
    last_pull: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<HashMap<PathBuf, SyncState>> {
    let mut states = HashMap::new();

    // Check all files in shade
    for entry in WalkDir::new(shade_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let rel_path = entry.path().strip_prefix(shade_dir)?;
        let local_path = project_path.join(rel_path);

        let local_meta = if local_path.exists() {
            Some(FileMetadata::from_path(&local_path)?)
        } else {
            None
        };

        let remote_meta = Some(FileMetadata::from_path(entry.path())?);

        let state = detect_sync_state(local_meta, remote_meta, last_pull);
        states.insert(rel_path.to_path_buf(), state);
    }

    // Check for local-only files
    let patterns = read_exclude(project_path)?;
    for pattern in &patterns {
        let local_path = project_path.join(pattern);

        if local_path.is_file() {
            let rel_path = PathBuf::from(pattern);
            if !states.contains_key(&rel_path) {
                states.insert(rel_path, SyncState::LocalOnly);
            }
        } else if local_path.is_dir() {
            for entry in WalkDir::new(&local_path) {
                let entry = entry?;
                if !entry.file_type().is_file() {
                    continue;
                }

                if let Ok(rel) = entry.path().strip_prefix(project_path) {
                    if !states.contains_key(rel) {
                        states.insert(rel.to_path_buf(), SyncState::LocalOnly);
                    }
                }
            }
        }
    }

    Ok(states)
}
```

**HashMap comparison:**

```rust
// Rust - HashMap (like Ruby Hash, Go map, JS Object/Map)
use std::collections::HashMap;

let mut states = HashMap::new();
states.insert(path.clone(), SyncState::InSync);

if let Some(state) = states.get(&path) {
    // use state
}

// Iteration
for (path, state) in &states {
    println!("{:?}: {:?}", path, state);
}
```

```ruby
# Ruby - Hash
states = {}
states[path] = :in_sync

if state = states[path]
  # use state
end

states.each do |path, state|
  puts "#{path}: #{state}"
end
```

```go
// Go - map
states := make(map[string]SyncState)
states[path] = InSync

if state, ok := states[path]; ok {
    // use state
}

for path, state := range states {
    fmt.Printf("%s: %v\n", path, state)
}
```

```elixir
# Elixir - Map
states = %{}
states = Map.put(states, path, :in_sync)

case Map.fetch(states, path) do
  {:ok, state} -> # use state
  :error -> # not found
end

Enum.each(states, fn {path, state} ->
  IO.puts("#{path}: #{state}")
end)
```

Update `src/commands/mod.rs`:

```rust
pub mod init;
pub mod add;
pub mod push;
pub mod pull;
pub mod status;
```

Update `src/main.rs`:

```rust
Commands::Status => commands::status::run(),
```

---

### Day 13-14: Comprehensive Testing (6-8 hours)

#### Task 13.1: Integration Tests (4 hours)

Create `tests/integration/workflow_test.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

mod common;

#[test]
fn test_complete_workflow() {
    // Setup
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("myapp");
    fs::create_dir(&project_path).unwrap();

    // Create git repo
    common::git_init(&project_path);

    // Create shade repo
    let shade_root = temp.path().join("shade");
    fs::create_dir_all(&shade_root.join("projects")).unwrap();
    common::git_init(&shade_root.join("projects"));

    // Create test files
    fs::write(project_path.join("config.local"), "secret=value").unwrap();
    fs::create_dir(project_path.join("secrets")).unwrap();
    fs::write(project_path.join("secrets/api.key"), "key123").unwrap();

    // Set HOME to temp (so shade uses our temp shade repo)
    std::env::set_var("HOME", temp.path());

    // Test init
    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project_path)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized"));

    // Test add
    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project_path)
        .args(&["add", "config.local", "secrets/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added to .git/info/exclude"));

    // Verify files copied
    assert!(shade_root.join("projects/myapp/config.local").exists());
    assert!(shade_root.join("projects/myapp/secrets/api.key").exists());

    // Test status
    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project_path)
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("config.local"));

    // Test push
    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project_path)
        .args(&["push", "-m", "Test commit"])
        .assert()
        .success();

    // Modify file
    fs::write(project_path.join("config.local"), "secret=new_value").unwrap();

    // Test status shows local ahead
    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project_path)
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("local ahead"));
}
```

**Test helpers in `tests/common/mod.rs`:**

```rust
use std::path::Path;
use std::process::Command;

pub fn git_init(path: &Path) {
    Command::new("git")
        .args(&["init"])
        .current_dir(path)
        .output()
        .expect("Failed to git init");

    // Configure git (needed for commits)
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git");

    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git");
}

pub fn setup_test_project() -> (tempfile::TempDir, std::path::PathBuf) {
    let temp = tempfile::TempDir::new().unwrap();
    let project = temp.path().join("project");
    std::fs::create_dir(&project).unwrap();
    git_init(&project);
    (temp, project)
}
```

#### Task 13.2: Unit Tests for Core Logic (2 hours)

Add more tests to existing modules:

```rust
// In src/core/conflict.rs

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_local_ahead() {
        let now = Utc::now();
        let last_pull = now - Duration::hours(2);

        let local = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now - Duration::hours(1),  // Modified after last_pull
            size: 100,
        };

        let remote = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: last_pull - Duration::hours(1),  // Before last_pull
            size: 100,
        };

        let state = detect_sync_state(Some(local), Some(remote), Some(last_pull));
        assert_eq!(state, SyncState::LocalAhead);
    }

    #[test]
    fn test_remote_ahead() {
        let now = Utc::now();
        let last_pull = now - Duration::hours(2);

        let local = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: last_pull - Duration::hours(1),  // Before last_pull
            size: 100,
        };

        let remote = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now - Duration::hours(1),  // After last_pull
            size: 100,
        };

        let state = detect_sync_state(Some(local), Some(remote), Some(last_pull));
        assert_eq!(state, SyncState::RemoteAhead);
    }

    #[test]
    fn test_local_only() {
        let now = Utc::now();

        let local = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now,
            size: 100,
        };

        let state = detect_sync_state(Some(local), None, None);
        assert_eq!(state, SyncState::LocalOnly);
    }

    #[test]
    fn test_remote_only() {
        let now = Utc::now();

        let remote = FileMetadata {
            path: PathBuf::from("file.txt"),
            modified: now,
            size: 100,
        };

        let state = detect_sync_state(None, Some(remote), None);
        assert_eq!(state, SyncState::RemoteOnly);
    }
}
```

#### Task 13.3: Error Handling Tests (2 hours)

Create `tests/integration/error_test.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_init_fails_without_git() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();

    // No git init

    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not a git repository"));
}

#[test]
fn test_add_fails_without_init() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();

    common::git_init(&project);
    fs::write(project.join("file.txt"), "content").unwrap();

    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project)
        .args(&["add", "file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not initialized"));
}

#[test]
fn test_add_fails_file_not_found() {
    // Setup project with init
    let (temp, project) = common::setup_and_init();

    Command::cargo_bin("git-shade")
        .unwrap()
        .current_dir(&project)
        .args(&["add", "nonexistent.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}
```

**Run all tests:**

```bash
cargo test                    # Run all tests
cargo test --test workflow   # Run specific test file
cargo test test_init         # Run tests matching name
cargo test -- --nocapture    # Show println! output
```

---

### Day 15: Polish and Documentation (4-6 hours)

#### Task 15.1: Better Error Messages (2 hours)

Update `src/error.rs` with more context:

```rust
use thiserror::Error;
use std::path::PathBuf;
use colored::Colorize;

#[derive(Error, Debug)]
pub enum ShadeError {
    #[error("{}", format_not_git_repo())]
    NotGitRepo,

    #[error("{}", format_not_initialized())]
    NotInitialized,

    #[error("Project already initialized: {0}\n  Location: ~/.local/git-shade/")]
    AlreadyInitialized(String),

    #[error("{}", format_shade_repo_not_found())]
    ShadeRepoNotFound,

    #[error("File not found: {}\n  Current directory: {}", .0.display(), std::env::current_dir().unwrap().display())]
    FileNotFound(PathBuf),

    #[error("No files tracked.\n  Add files with: git-shade add <files>")]
    NoFilesTracked,

    #[error("{}", format_conflict_detected(.files))]
    ConflictDetected {
        files: Vec<String>,
    },

    #[error("Git command failed: {0}\n  Try: git status\n  Or: git remote -v")]
    GitError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

fn format_not_git_repo() -> String {
    format!(
        "{}\n  {}\n  {}",
        "Not a git repository.".red(),
        "Initialize with:".dimmed(),
        "git init".bold()
    )
}

fn format_not_initialized() -> String {
    format!(
        "{}\n  {}\n  {}",
        "Project not initialized.".red(),
        "Initialize with:".dimmed(),
        "git-shade init".bold()
    )
}

fn format_shade_repo_not_found() -> String {
    format!(
        "{}\n  {}\n  {}",
        "Shade repository not found at ~/.local/git-shade/projects/".red(),
        "Clone it first with:".dimmed(),
        "git clone <url> ~/.local/git-shade/projects".bold()
    )
}

fn format_conflict_detected(files: &[String]) -> String {
    let mut msg = format!("{}\n", "Conflicts detected.".red());
    msg.push_str(&format!("  Files in conflict:\n"));
    for file in files {
        msg.push_str(&format!("    - {}\n", file));
    }
    msg.push_str(&format!("\n  {}\n", "Manual resolution required.".yellow()));
    msg.push_str(&format!("  Use: {}", "git-shade pull --force".bold()));
    msg
}

pub type Result<T> = std::result::Result<T, ShadeError>;
```

#### Task 15.2: Add Helpful Hints (1 hour)

Update commands to show next steps:

```rust
// In src/commands/add.rs at the end
println!();
println!("{}", "Next steps:".bold());
println!("  1. Review changes: {}", "git-shade status".bold());
println!("  2. Sync to other machines: {}", "git-shade push".bold());
```

```rust
// In src/commands/push.rs at the end
println!();
println!("{}", "✓ Files synced to shade repo".green());
println!();
println!("On other machines:");
println!("  1. Clone shade repo: {}",
    format!("git clone <url> ~/.local/git-shade/projects").bold());
println!("  2. Pull files: {}", "git-shade pull".bold());
```

#### Task 15.3: README.md (2 hours)

Create comprehensive README:

```markdown
# git-shade

Sync git-excluded files across multiple machines.

## Problem

Files in `.gitignore` or `.git/info/exclude` (local configs, secrets, large files) don't sync between machines, forcing manual recreation.

## Solution

git-shade maintains a unified Git repository that syncs your excluded files while keeping them out of your main project repository.

## Installation

```bash
cargo install git-shade
```

Or build from source:

```bash
git clone https://github.com/yourusername/git-shade
cd git-shade
cargo build --release
sudo cp target/release/git-shade /usr/local/bin/
```

## Quick Start

### First Machine

```bash
# 1. Create and clone your unified shade repo
mkdir -p ~/.local/git-shade
cd ~/.local/git-shade
git clone git@github.com:user/my-shade-files.git projects

# 2. In your project
cd ~/projects/myapp
git-shade init

# 3. Add files to sync
git-shade add config.local secrets/

# 4. Push to shade repo
git-shade push
```

### Second Machine

```bash
# 1. Clone shade repo
git clone git@github.com:user/my-shade-files.git ~/.local/git-shade/projects

# 2. Clone your project
git clone git@github.com:user/myapp.git ~/projects/myapp
cd ~/projects/myapp

# 3. Init and pull
git-shade init
# Automatically detects and pulls files!
```

## Commands

### `git-shade init`

Initialize shade for current project.

```bash
git-shade init [--name <name>]
```

### `git-shade add <files...>`

Add files/directories to shade.

```bash
git-shade add config.local
git-shade add secrets/ .env.local
```

### `git-shade push`

Sync local changes to shade repo.

```bash
git-shade push
git-shade push -m "Updated database config"
```

### `git-shade pull`

Pull changes from shade repo.

```bash
git-shade pull
git-shade pull --force      # Overwrite local changes
git-shade pull --dry-run    # Preview changes
```

### `git-shade status`

Show sync status of files.

```bash
git-shade status
```

## How It Works

```
~/.local/git-shade/
  config.toml          # Global config
  metadata/            # Per-project metadata
    myapp/.shade-sync  # Timestamps
  projects/            # Unified Git repo
    myapp/             # Your shade files
      config.local
      secrets/
    another-app/
      .env.local
    .git/              # Single repo for all projects
```

## Conflict Resolution

If both local and remote versions changed:

```bash
$ git-shade pull
⚠ CONFLICTS DETECTED

  ⚠ config.local
    Local:  modified 2025-10-11 14:25:00
    Remote: modified 2025-10-11 14:30:00

Manual resolution required.
```

Options:
1. **Force overwrite**: `git-shade pull --force`
2. **Keep local**: `git-shade push`
3. **Manual merge**: Review files in `~/.local/git-shade/projects/myapp/`

## FAQ

**Q: What files should I put in shade?**

A: Local configs, development secrets, large binary files, machine-specific settings.

**Q: Is it secure?**

A: Your shade repo is a private Git repository. Use SSH keys and keep the repo private.

**Q: Can I use multiple shade repos?**

A: Currently one unified repo for all projects. This keeps setup simple.

**Q: What if I delete a file locally?**

A: `git-shade status` shows it as "remote only". Pull to restore or manually delete from shade.

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- status

# Build release
cargo build --release
```

## License

MIT
```

#### Task 15.4: Examples and Help Text (1 hour)

Create `examples/basic_workflow.md`:

```markdown
# Basic Workflow Examples

## Setup New Project

```bash
cd ~/projects/myapp

# Initialize git-shade
git-shade init

# Add files
git-shade add config.local .env.local secrets/

# Check status
git-shade status

# Push to shade repo
git-shade push -m "Initial shade setup"
```

## Daily Workflow

```bash
# Morning: pull latest
git-shade pull

# Work on project...
vim config.local

# End of day: push changes
git-shade push

# Or check what changed first
git-shade status
```

## Setup on New Machine

```bash
# Clone shade repo (one time)
git clone git@github.com:user/my-shade.git ~/.local/git-shade/projects

# For each project
cd ~/projects/myapp
git-shade init  # Auto-pulls files

# Or if you declined auto-pull
git-shade pull
```

## Conflict Resolution

```bash
# Situation: both machines modified same file

# Machine A
echo "version=1.0" > config.local
git-shade push

# Machine B (meanwhile)
echo "version=2.0" > config.local
git-shade pull
# ⚠ CONFLICTS DETECTED

# Option 1: Keep local
git-shade push  # Your version wins

# Option 2: Take remote
git-shade pull --force  # Remote version wins

# Option 3: Manual merge
cd ~/.local/git-shade/projects/myapp
vim config.local  # Merge manually
cp config.local ~/projects/myapp/
cd ~/projects/myapp
git-shade push
```

## Advanced: Multiple Projects

```bash
# All projects share one shade repo

# Project 1
cd ~/projects/webapp
git-shade init
git-shade add .env.local
git-shade push

# Project 2
cd ~/projects/api
git-shade init
git-shade add secrets/
git-shade push

# Shade repo contains both:
# ~/.local/git-shade/projects/
#   webapp/.env.local
#   api/secrets/
#   .git/
```
```

---

## Week 4: Distribution and Final Polish

### Day 16-17: Cargo Publishing Preparation (4-6 hours)

#### Task 16.1: Cargo.toml Metadata (1 hour)

Update `Cargo.toml` with complete metadata:

```toml
[package]
name = "git-shade"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Sync git-excluded files across multiple machines"
readme = "README.md"
homepage = "https://github.com/yourusername/git-shade"
repository = "https://github.com/yourusername/git-shade"
license = "MIT"
keywords = ["git", "sync", "config", "secrets", "dotfiles"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.5", features = ["derive", "cargo"] }
colored = "2.1"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
walkdir = "2.5"
dirs = "5.0"
dialoguer = "0.11"
env_logger = "0.11"
log = "0.4"

[dev-dependencies]
tempfile = "3.10"
assert_cmd = "2.0"
predicates = "3.1"
serial_test = "3.0"

[profile.release]
strip = true          # Remove debug symbols
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
opt-level = "z"      # Optimize for size
```

#### Task 16.2: CLI Help Text (2 hours)

Enhance `src/cli.rs` with better help:

```rust
use clap::{Parser, Subcommand};

/// Sync git-excluded files across multiple machines
///
/// git-shade maintains a unified Git repository that syncs your
/// excluded files while keeping them out of your main repository.
///
/// Quick start:
///   1. Clone shade repo: git clone <url> ~/.local/git-shade/projects
///   2. Initialize project: git-shade init
///   3. Add files: git-shade add config.local
///   4. Push to sync: git-shade push
#[derive(Parser)]
#[command(name = "git-shade")]
#[command(author, version, about, long_about)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize shade for current project
    ///
    /// Creates metadata and detects existing shade files.
    /// If shade repo contains files for this project, offers to pull them.
    ///
    /// Example:
    ///   git-shade init
    ///   git-shade init --name my-custom-name
    Init {
        /// Custom project name (default: directory name)
        #[arg(long)]
        name: Option<String>,
    },

    /// Add files/directories to shade
    ///
    /// Adds patterns to .git/info/exclude and copies files
    /// to shade repository. Preserves directory structure.
    ///
    /// Examples:
    ///   git-shade add config.local
    ///   git-shade add secrets/ .env.local
    Add {
        /// Files or directories to add
        #[arg(required = true)]
        files: Vec<String>,
    },

    /// Sync local changes to shade repo
    ///
    /// Copies tracked files to shade, commits, and pushes to remote.
    ///
    /// Examples:
    ///   git-shade push
    ///   git-shade push -m "Updated config"
    Push {
        /// Custom commit message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Pull changes from shade repo
    ///
    /// Runs git pull in shade repo, detects conflicts,
    /// and safely syncs files to local project.
    ///
    /// Examples:
    ///   git-shade pull
    ///   git-shade pull --force       # Overwrite local
    ///   git-shade pull --dry-run     # Preview
    Pull {
        /// Overwrite local files without conflict checking
        #[arg(long)]
        force: bool,

        /// Show what would happen without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// Show synchronization status
    ///
    /// Displays state of all tracked files:
    ///   ✓ in sync    ↑ local ahead    ↓ remote ahead
    ///   ⚠ conflict   ? local only     ← remote only
    ///
    /// Example:
    ///   git-shade status
    Status,
}
```

#### Task 16.3: Logging (1 hour)

Add logging support:

Update `src/main.rs`:

```rust
mod cli;
mod error;
mod core;
mod utils;
mod commands;
mod git;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use log::{debug, info};

fn main() {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose {
        "debug"
    } else {
        "info"
    };

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(log_level)
    ).init();

    debug!("Starting git-shade");

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { name } => {
            info!("Running init command");
            commands::init::run(name)
        },
        Commands::Add { files } => {
            info!("Running add command with {} files", files.len());
            commands::add::run(files)
        },
        Commands::Push { message } => {
            info!("Running push command");
            commands::push::run(message)
        },
        Commands::Pull { force, dry_run } => {
            info!("Running pull command (force={}, dry_run={})", force, dry_run);
            commands::pull::run(force, dry_run)
        },
        Commands::Status => {
            info!("Running status command");
            commands::status::run()
        },
    }
}
```

Usage:

```bash
# Normal output
git-shade status

# Verbose (shows debug logs)
git-shade --verbose status

# Or with env var
RUST_LOG=debug git-shade status
```

#### Task 16.4: CI/CD Setup (2 hours)

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        profile: minimal
        override: true
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache target directory
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

    - name: Run tests
      run: cargo test --verbose

    - name: Check formatting
      run: cargo fmt -- --check

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Build release
      run: cargo build --release --verbose

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin

    - name: Generate coverage
      run: cargo tarpaulin --out Xml

    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            artifact_name: git-shade
            asset_name: git-shade-linux-amd64
          - os: macos-latest
            artifact_name: git-shade
            asset_name: git-shade-macos-amd64
          - os: windows-latest
            artifact_name: git-shade.exe
            asset_name: git-shade-windows-amd64.exe

    steps:
    - uses: actions/checkout@v3

    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        profile: minimal
        override: true

    - name: Build release
      run: cargo build --release

    - name: Upload binary
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ github.event.release.upload_url }}
        asset_path: ./target/release/${{ matrix.artifact_name }}
        asset_name: ${{ matrix.asset_name }}
        asset_content_type: application/octet-stream
```

---

### Day 18: Final Testing and Documentation (4-6 hours)

#### Task 18.1: Manual Testing Checklist (2 hours)

Create `TESTING.md`:

```markdown
# Testing Checklist

## Setup Tests

- [ ] First machine init works
- [ ] Second machine init detects files
- [ ] Auto-pull on init works
- [ ] Decline auto-pull works
- [ ] Custom project name works

## Add Tests

- [ ] Add single file
- [ ] Add multiple files
- [ ] Add directory
- [ ] Add nested directory
- [ ] Adds to .git/info/exclude
- [ ] Doesn't duplicate in exclude
- [ ] Files copied to shade

## Push Tests

- [ ] Push with no message
- [ ] Push with custom message
- [ ] Push to remote works
- [ ] Push without remote shows warning
- [ ] Updates last_push timestamp
- [ ] Handles no tracked files

## Pull Tests

- [ ] Pull when in sync
- [ ] Pull with remote ahead
- [ ] Pull with local ahead (skips)
- [ ] Pull with conflict (aborts)
- [ ] Pull --force overwrites
- [ ] Pull --dry-run shows plan
- [ ] Updates last_pull timestamp
- [ ] Adds new files to exclude

## Status Tests

- [ ] Shows in sync files
- [ ] Shows local ahead
- [ ] Shows remote ahead
- [ ] Shows conflicts
- [ ] Shows local only
- [ ] Shows remote only
- [ ] Shows git remote info
- [ ] Shows timestamps

## Error Cases

- [ ] Init without git repo
- [ ] Init when already initialized
- [ ] Init without shade repo cloned
- [ ] Add file that doesn't exist
- [ ] Add without init
- [ ] Push without init
- [ ] Push without tracked files
- [ ] Pull without init

## Edge Cases

- [ ] Files with spaces in name
- [ ] Files with special characters
- [ ] Deep directory nesting
- [ ] Large files (>10MB)
- [ ] Many files (>100)
- [ ] Empty directories
- [ ] Symlinks
```

Manually test each:

```bash
# Setup test environment
mkdir -p /tmp/shade-test
cd /tmp/shade-test

# Create shade repo
mkdir -p shade/projects
cd shade/projects
git init
git config user.email "test@example.com"
git config user.name "Test User"
cd ../..

# Create project
mkdir project1
cd project1
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Test init
HOME=/tmp/shade-test git-shade init

# Test add
echo "secret" > config.local
HOME=/tmp/shade-test git-shade add config.local

# Test status
HOME=/tmp/shade-test git-shade status

# Test push
cd /tmp/shade-test/shade/projects
git config user.email "test@example.com"
git config user.name "Test User"
cd /tmp/shade-test/project1
HOME=/tmp/shade-test git-shade push

# etc...
```

#### Task 18.2: Performance Testing (1 hour)

Create `benches/file_sync.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use git_shade::utils::copy_file_preserve_structure;
use tempfile::TempDir;
use std::fs;

fn bench_copy_small_file(c: &mut Criterion) {
    c.bench_function("copy small file", |b| {
        b.iter(|| {
            let temp = TempDir::new().unwrap();
            let src = temp.path().join("src");
            let dst = temp.path().join("dst");
            fs::create_dir_all(&src).unwrap();
            fs::write(src.join("file.txt"), "content").unwrap();

            copy_file_preserve_structure(
                &src.join("file.txt"),
                &src,
                &dst
            ).unwrap();
        });
    });
}

fn bench_copy_large_file(c: &mut Criterion) {
    c.bench_function("copy large file (1MB)", |b| {
        let content = vec![0u8; 1024 * 1024]; // 1MB

        b.iter(|| {
            let temp = TempDir::new().unwrap();
            let src = temp.path().join("src");
            let dst = temp.path().join("dst");
            fs::create_dir_all(&src).unwrap();
            fs::write(src.join("large.bin"), &content).unwrap();

            copy_file_preserve_structure(
                &src.join("large.bin"),
                &src,
                &dst
            ).unwrap();
        });
    });
}

criterion_group!(benches, bench_copy_small_file, bench_copy_large_file);
criterion_main!(benches);
```

Add to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "file_sync"
harness = false
```

Run benchmarks:

```bash
cargo bench
```

#### Task 18.3: Documentation Polish (2 hours)

Add inline documentation:

```rust
// src/commands/init.rs

//! Project initialization command.
//!
//! This module handles the `git-shade init` command which:
//! - Verifies the current directory is a git repository
//! - Creates shade metadata structure
//! - Detects and optionally pulls existing shade files

/// Initialize shade for the current project.
///
/// # Arguments
///
/// * `name_override` - Optional custom project name. If None, uses directory name.
///
/// # Errors
///
/// Returns an error if:
/// - Current directory is not a git repository
/// - Project is already initialized
/// - Shade repository is not cloned
///
/// # Examples
///
/// ```no_run
/// use git_shade::commands::init;
///
/// // Use directory name
/// init::run(None)?;
///
/// // Custom name
/// init::run(Some("my-project".to_string()))?;
/// # Ok::<(), git_shade::error::ShadeError>(())
/// ```
pub fn run(name_override: Option<String>) -> Result<()> {
    // Implementation...
}
```

Generate docs:

```bash
cargo doc --open
```

#### Task 18.4: CONTRIBUTING.md (1 hour)

Create `CONTRIBUTING.md`:

```markdown
# Contributing to git-shade

Thanks for your interest! This guide will help you get started.

## Development Setup

```bash
# Clone
git clone https://github.com/yourusername/git-shade
cd git-shade

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- --help
```

## Code Style

We use standard Rust formatting:

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check before committing
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Project Structure

- `src/commands/` - Command implementations
- `src/core/` - Core logic (config, sync, conflict detection)
- `src/git/` - Git operations wrapper
- `src/utils/` - Helper utilities
- `tests/` - Integration tests

## Adding a New Feature

1. **Write tests first**
   ```rust
   #[test]
   fn test_new_feature() {
       // Test code
   }
   ```

2. **Implement the feature**
   - Add to appropriate module
   - Update CLI if needed
   - Add documentation

3. **Update README**
   - Add usage examples
   - Update command documentation

4. **Submit PR**
   - Clear description
   - Reference any issues
   - Include tests

## Common Tasks

### Adding a new command

1. Create `src/commands/mycommand.rs`
2. Add to `src/commands/mod.rs`
3. Add to `src/cli.rs`
4. Add to match in `src/main.rs`
5. Write tests in `tests/integration/mycommand_test.rs`

### Debugging

```bash
# Run with logging
RUST_LOG=debug cargo run -- status

# Run specific test with output
cargo test test_name -- --nocapture
```

## Questions?

Open an issue or discussion on GitHub!
```

---

### Day 19-20: Publishing and Polish (4-6 hours)

#### Task 19.1: Publish to crates.io (2 hours)

```bash
# Login to crates.io
cargo login

# Dry run
cargo publish --dry-run

# Publish
cargo publish
```

**Pre-publish checklist:**
- [ ] All tests pass
- [ ] Documentation is complete
- [ ] README is comprehensive
- [ ] License file exists
- [ ] Cargo.toml metadata is complete
- [ ] Version number is correct
- [ ] Git tag matches version

#### Task 19.2: Create Installation Guide (1 hour)

Update README with installation:

```markdown
## Installation

### From crates.io (recommended)

```bash
cargo install git-shade
```

### From source

```bash
git clone https://github.com/yourusername/git-shade
cd git-shade
cargo install --path .
```

### Pre-built binaries

Download from [Releases](https://github.com/yourusername/git-shade/releases):

- **Linux**: `git-shade-linux-amd64`
- **macOS**: `git-shade-macos-amd64`
- **Windows**: `git-shade-windows-amd64.exe`

```bash
# Linux/macOS
chmod +x git-shade-*
sudo mv git-shade-* /usr/local/bin/git-shade

# Verify
git-shade --version
```

## First-Time Setup

```bash
# 1. Create shade repository (GitHub/GitLab/etc)
# (On GitHub: New Repository → my-shade-files → Private)

# 2. Clone it locally
git clone git@github.com:user/my-shade-files.git ~/.local/git-shade/projects

# 3. In your project
cd ~/projects/myapp
git-shade init
git-shade add config.local secrets/
git-shade push

# Done! Your files are synced.
```
```

#### Task 19.3: Create Demo Video Script (1 hour)

Create `DEMO.md`:

```markdown
# Demo Script

## Setup (30 seconds)

```bash
# Show: Clean environment
ls ~/.local/git-shade/  # Doesn't exist yet

# Install
cargo install git-shade

# Create shade repo
git clone git@github.com:demo/shade.git ~/.local/git-shade/projects
```

## First Project (1 minute)

```bash
# Show: Existing project
cd ~/projects/webapp
ls
# config.local  src/  .git/

# Initialize
git-shade init
# ✓ Initialized git-shade for project: webapp

# Add files
git-shade add config.local
# ✓ Added to .git/info/exclude
# ✓ Copied to ~/.local/git-shade/projects/webapp/

# Show: Files excluded from git
git status
# (config.local not shown)

# Push to shade
git-shade push
# ✓ Pushed to origin/main
```

## Second Machine (1 minute)

```bash
# Show: New machine
ssh laptop

# Clone shade repo
git clone git@github.com:demo/shade.git ~/.local/git-shade/projects

# Clone project (no shade files yet)
git clone git@github.com:demo/webapp.git ~/projects/webapp
cd ~/projects/webapp
ls
# src/  .git/
# (no config.local!)

# Initialize shade
git-shade init
# Found 1 file in shade:
#   - config.local
# Pull these files now? [Y/n]: y
# ✓ config.local

# Show: File restored
ls
# config.local  src/  .git/
cat config.local
# (shows content)
```

## Daily Workflow (30 seconds)

```bash
# Morning: Pull latest
git-shade pull
# ✓ Pull completed

# Work...
vim config.local

# Evening: Push changes
git-shade push
# ✓ Pushed to origin/main

# Status anytime
git-shade status
# Files:
#   ✓ config.local (in sync)
```

## Conflict Handling (1 minute)

```bash
# Simulate: Two machines edit same file

# Machine A
echo "version=1.0" >> config.local
git-shade push

# Machine B (meanwhile)
echo "version=2.0" >> config.local
git-shade pull
# ⚠ CONFLICTS DETECTED
#   ⚠ config.local

# Resolve: Force
git-shade pull --force
# ✓ config.local (overwritten)

cat config.local
# version=1.0  (from machine A)
```
```

#### Task 19.4: Final Checklist (1 hour)

Create `RELEASE_CHECKLIST.md`:

```markdown
# Release Checklist

## Pre-Release

- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt -- --check`)
- [ ] Documentation builds (`cargo doc`)
- [ ] README is up to date
- [ ] CHANGELOG.md is updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created (`git tag v0.1.0`)

## Testing

- [ ] Manual testing completed (see TESTING.md)
- [ ] Tested on Linux
- [ ] Tested on macOS
- [ ] Tested on Windows
- [ ] Integration tests pass
- [ ] Benchmarks run successfully

## Documentation

- [ ] README has install instructions
- [ ] All commands documented
- [ ] Examples are accurate
- [ ] CONTRIBUTING.md exists
- [ ] API docs are complete

## Publishing

- [ ] `cargo publish --dry-run` succeeds
- [ ] Published to crates.io
- [ ] GitHub release created
- [ ] Binaries attached to release
- [ ] Announced on social media/forums

## Post-Release

- [ ] Update version to next dev version
- [ ] Create milestone for next release
- [ ] Close completed issues
- [ ] Update project board
```

---

## Summary: Complete Timeline

### Week 1: Foundation
- **Day 1**: Project setup, CLI basics, Rust fundamentals
- **Day 2**: Error handling, testing framework
- **Day 3**: Paths, configuration, file structures
- **Day 4**: Tracker, `init` command

### Week 2: Core Features
- **Day 5**: `add` command, file operations
- **Day 6-7**: `push` command, git operations
- **Day 8-10**: `pull` command, conflict detection

### Week 3: Polish
- **Day 11-12**: `status` command
- **Day 13-14**: Comprehensive testing
- **Day 15**: Error messages, documentation, README

### Week 4: Release
- **Day 16-17**: Publishing prep, CI/CD, metadata
- **Day 18**: Final testing, performance
- **Day 19-20**: Publish, installation guides, demos

---

## Key Rust Concepts Summary

### Ownership & Borrowing
```rust
let s = String::from("hello");  // s owns the string
let r = &s;                      // r borrows s (read-only)
let r2 = &mut s;                 // mutable borrow
```

### Pattern Matching
```rust
