# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Proact is a Rust CLI tool that generates comprehensive documentation for AI coding agents (Claude Code, Gemini CLI, OpenCode, etc.). The generated documentation instructs AI agents to follow best practices, apply continuous improvement feedback, and utilize tools like Playwright MCP for browser automation.

## CLI Usage

```bash
# Basic usage
cargo run -- <target-project-path>

# With verbose output (shows file operations like mkdir and write)
cargo run -- -v <target-project-path>

# Dry run mode (preview operations without creating files)
cargo run -- -n <target-project-path>
cargo run -- --dry-run <target-project-path>

# Specify custom output directory
cargo run -- -o ./custom-docs <target-project-path>

# Example: Generate docs for a neighboring project
cargo run -- -v ../my-project

# Example: Preview what would be created without actually creating files
cargo run -- -n -o ./test-docs ../my-project

# Build release version
cargo build --release

# Run release version with dry-run
./target/release/proact -n ../my-project
```

## Build and Development Commands

### Rust Commands
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Check for compilation errors without building
cargo check

# Build documentation
cargo doc --open
```

## Architecture and Structure

### Core Components
- **src/main.rs**: CLI entry point that handles argument parsing and orchestrates document generation
- **src/cli.rs**: Command-line argument parsing using clap, defines CLI interface
- **src/generator.rs**: Core logic for generating documentation, detects project type and customizes output
- **src/templates.rs**: Contains all documentation templates (process guidelines, quality standards, Playwright MCP setup)
- **docs/ai_agent_guidelines.md**: Source guidelines for AI agent development processes
- **research/**: Playwright MCP server documentation and setup guides used as reference

### Development Process (Checkpoints)

When reaching a checkpoint in development, follow this sequence:
1. Run and fix all failing tests (`cargo test`)
2. Fix linting issues (`cargo clippy`)
3. Format source code (`cargo fmt`)
4. Update documentation as needed
5. Manage git status appropriately
6. Create logical commits with clear messages
7. Push changes immediately after committing

### Generated Documentation Features

The CLI generates documentation that includes:
- **Process Guidelines**: Checkpoint-based development workflow for AI agents
- **Quality Standards**: Documentation, testing, and code quality requirements  
- **Continuous Improvement**: Learning from failures and updating processes
- **Playwright MCP Setup**: Installation and usage instructions for browser automation
- **Project-Specific Notes**: Automatically detects project type (Rust, JavaScript, Python, Go) and adds relevant commands

### Quality Standards

- Include unit tests for individual functions
- Provide functional tests for requirements verification
- Write integration tests for module interoperability
- Document all modules and functions with doc comments
- Maintain doc tests within code comments