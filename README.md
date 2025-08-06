# Proact

A CLI tool that generates comprehensive documentation for AI coding agents (Claude Code, Gemini CLI, OpenCode, etc.).

## Purpose

Proact creates documentation that instructs AI coding agents to:
- Follow best development practices and processes
- Apply continuous improvement based on feedback
- Understand how to install and use tools like Playwright MCP
- Maintain code quality through testing, linting, and documentation

## Usage

```bash
# Basic usage
cargo run -- <target-project-path>

# With verbose output (shows file operations)
cargo run -- -v <target-project-path>

# Dry run mode (preview without creating files)
cargo run -- -n <target-project-path>

# Specify custom output directory
cargo run -- -o ./custom-docs <target-project-path>

# Example: Generate docs for a neighboring project
cargo run -- -v ../my-project

# Example: Preview what would be created
cargo run -- --dry-run ../my-project
```

## CLI Options

- `-V, --version` - Show version information
- `-h, --help` - Display help message
- `-v, --verbose` - Enable verbose output showing generation progress and file operations
- `-n, --dry-run` - Show what would be done without actually creating files (implies verbose)
- `-o, --output-dir <DIR>` - Output directory for generated documentation (default: `./docs`)
- `<TARGET>` - Required: Path to an existing project directory

## Generated Documentation

The tool generates an `ai_agent_instructions.md` file containing:

1. **Process Guidelines** - Checkpoint-based development workflow
2. **Quality Standards** - Documentation, testing, and code quality requirements
3. **Continuous Improvement** - Learning from failures and updating processes
4. **Playwright MCP Setup** - Installation and usage instructions for browser automation
5. **Project-Specific Notes** - Automatically detects project type and adds relevant commands

## Project Detection

Proact automatically detects the project type and includes appropriate commands:
- **Rust** - cargo commands for build, test, clippy, fmt
- **JavaScript/Node.js** - npm/yarn commands for dependencies, testing, linting
- **Python** - pip/poetry, pytest, black, pylint commands
- **Go** - go build, test, vet, fmt commands

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Build release version
cargo build --release
```

## Future Enhancements

Future versions will include:
- Additional MCP server documentation beyond Playwright
- More language-specific guidelines
- Customizable templates
- Configuration file support

## License

MIT