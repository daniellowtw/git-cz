# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `git-cz`, a Rust-based commitizen CLI tool that helps create conventional commits. The tool provides an interactive interface for selecting commit types, scopes, and descriptions, then formats them according to the conventional commits specification.

## Common Commands

### Build and Development
- `cargo build` - Build the project
- `cargo run` - Run the CLI tool locally
- `cargo test` - Run all tests
- `cargo install --path .` - Install the binary locally

### Release and Publishing
- `cargo dist build` - Build distribution packages
- `release-plz release` - Automated release process using release-plz

### Changelog Generation
- `git-cliff` - Generate changelog using the cliff.toml configuration

## Code Architecture

### Core Components

**main.rs** - Entry point with interactive prompts using promkit:
- Displays commit type selection menu
- Prompts for scope, description, body, and optional footer
- Supports editor integration for longer commit bodies
- Handles confirmation before committing

**lib.rs** - Core library functions:
- `build_commit_types()` - Defines conventional commit types (feat, fix, docs, etc.)
- `format_commit_types()` - Formats commit types for display with proper alignment
- `build_commit_message()` - Constructs the final commit message from components
- `perform_commit()` - Executes the git commit using git2 library

### Key Dependencies
- `git2` - Git repository operations and committing
- `promkit` - Interactive CLI prompts and menus
- `tempfile` - Temporary file handling for editor integration

### Conventional Commit Types
The tool supports standard conventional commit types: feat, fix, docs, style, refactor, perf, test, chore, ci, build, revert.

### Testing
Comprehensive test suite in `tests/main_test.rs` covers:
- Commit type building and formatting
- Commit message construction with various scenarios
- Git operations with temporary repositories
- Edge cases and error conditions

## Configuration Files

- `cliff.toml` - git-cliff configuration for changelog generation
- `release-plz.toml` - Automated release configuration
- `Cargo.toml` - Standard Rust project configuration with cargo-dist setup

## Development Notes

The binary is named `git-cz` and integrates with git as a subcommand (allowing `git cz` usage). The tool validates staged changes exist before committing and uses git2 for all repository operations.