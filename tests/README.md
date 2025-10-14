# git-shade Integration Tests

This directory contains integration tests for git-shade.

## Test Coverage

### Basic Commands
- [x] `git-shade init` - Initialize a project
- [x] `git-shade add` - Add files to shade
- [x] `git-shade push` - Push files to shade repo
- [x] `git-shade pull` - Pull files from shade repo
- [x] `git-shade status` - Show file sync status

### Workflows
- [ ] End-to-end: init → add → push → pull on second machine
- [ ] Conflict detection and resolution
- [ ] Multiple projects in same shade repo
- [ ] File modification scenarios

### Edge Cases
- [ ] Missing files
- [ ] Permission errors
- [ ] Git conflicts
- [ ] Network failures

## Running Tests

```bash
# Run all tests
cargo test

# Run integration tests only
cargo test --test integration_test

# Run specific test
cargo test test_help_works
```

## Test Structure

```
tests/
  integration/          # Future integration tests
    workflow_test.rs
    conflict_test.rs
    multi_project_test.rs
  common/              # Shared test utilities
    mod.rs
```
