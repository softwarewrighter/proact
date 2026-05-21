# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Proact is a Rust CLI tool that generates comprehensive documentation for AI coding agents (Claude Code, Gemini CLI, OpenCode, etc.). The generated documentation instructs AI agents to follow best practices, apply continuous improvement feedback, and utilize tools like Playwright MCP for browser automation.

## CLI Usage

```bash
# Build release version (from proact-cli component)
cd components/proact-cli && cargo build --release

# Basic usage
./components/proact-cli/target/release/proact <target-project-path>

# Show version with copyright, license, and repository
./components/proact-cli/target/release/proact -V

# Show extended help with AI coding agent guidance
./components/proact-cli/target/release/proact --help

# With verbose output (shows file operations like mkdir and write)
./components/proact-cli/target/release/proact -v <target-project-path>

# Dry run mode (preview operations without creating files)
./components/proact-cli/target/release/proact -n <target-project-path>

# Specify custom output directory
./components/proact-cli/target/release/proact -o ./custom-docs <target-project-path>
```

## Build and Development Commands

```bash
# Build all components
./scripts/build-all.sh

# Run tests and clippy on all components
./scripts/check-all.sh

# Format all components
./scripts/fmt-all.sh

# Run proact CLI
./scripts/run.sh <target-project-path>
./scripts/run.sh -V
./scripts/run.sh --help

# Single component operations
cd components/proact-cli && cargo build --release
cd components/proact-cli && cargo test
cd components/proact-cli && cargo clippy --all-targets --all-features -- -D warnings
cd components/proact-cli && cargo fmt
```

## Architecture and Structure

This project follows the Software Wrighter modular architecture pattern with no Cargo.toml in the repo root.

### Component Structure

```
proact/
├── components/
│   ├── proact-cli/              # Main CLI component (workspace)
│   │   ├── Cargo.toml           # Workspace manifest
│   │   └── crates/
│   │       ├── proact/          # Binary entry point
│   │       ├── cli-args/        # Argument parsing (clap)
│   │       ├── cli-output/      # Output formatting
│   │       └── cli-generator/   # Documentation generation
│   │
│   ├── proact-templates/        # Template handling (workspace)
│   │   └── crates/
│   │       ├── template-guidelines/  # Process/quality templates
│   │       └── template-notes/       # Project-specific templates
│   │
│   ├── proact-metadata/         # Metadata extraction (workspace)
│   │   └── crates/
│   │       ├── metadata-core/   # Core types, MIT license
│   │       └── metadata-extract/# Git/file extraction
│   │
│   └── proact-run/              # Runtime helpers (workspace)
│       └── crates/
│           ├── run-files/       # File operations
│           └── run-learnings/   # Learnings management
│
├── templates/                   # Template .md files
│   ├── process_guidelines.md
│   ├── quality_standards.md
│   ├── continuous_improvement.md
│   ├── code_metrics.md
│   ├── playwright_mcp_setup.md
│   ├── summary.md
│   ├── rust_notes.md
│   ├── javascript_notes.md
│   ├── python_notes.md
│   ├── process.md
│   ├── tools.md
│   └── youtube.md
│
└── docs/
```

### Modularity Constraints

Each component follows sw-checklist standards:
- ≤4 functions per module
- ≤4 modules per crate
- ≤4 crates per component
- ≤25 lines per function (warning), ≤50 lines (failure)

### Development Process (Checkpoints)

When reaching a checkpoint in development, follow this sequence:
1. Run and fix all failing tests (`cargo test` in each component)
2. Fix linting issues (`cargo clippy`)
3. Format source code (`cargo fmt`)
4. Update documentation as needed
5. Manage git status appropriately
6. Create logical commits with clear messages
7. Push changes immediately after committing

### Generated Documentation Features

The CLI generates the following documentation in the target project's docs directory:

1. **ai_agent_instructions.md**: Comprehensive AI agent guidelines
   - Process Guidelines: Checkpoint-based development workflow
   - Quality Standards: Documentation, testing, and code quality requirements
   - Continuous Improvement: Learning from failures and updating processes
   - Code Metric Gates: Small-part complexity gates (≤25 LOC/fn, ≤5 fns/module, ≤5 modules/crate) and refactoring playbook
   - Playwright MCP Setup: Installation and usage instructions for browser automation
   - Project-Specific Notes: Automatically detects project type (Rust, JavaScript, Python) and adds relevant commands

2. **process.md**: Detailed development process workflow

3. **tools.md**: Development tools reference

4. **code_metrics.md**: Standalone copy of the code metric gates and architecture guide

5. **COPYRIGHT**: Copyright notice (dynamically generated)

6. **LICENSE**: MIT License file (dynamically generated)

7. **learnings.md**: Continuous improvement tracking

### Quality Standards

- Include unit tests for individual functions
- Provide functional tests for requirements verification
- Write integration tests for module interoperability
- Document all modules and functions with doc comments
- Maintain doc tests within code comments
