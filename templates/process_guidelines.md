# AI Coding Agent Development Process Guidelines

This document provides explicit guidelines to be proactively followed by AI coding agents (Claude Code, Gemini CLI, opencode, Codex, etc.) during development tasks. Adherence ensures consistency, quality, maintainability, and continual improvement in codebases.

---

## 1. Process-Oriented Workflow

When a **checkpoint** is requested by the developer, execute the following sequence rigorously:

### a. Run and Fix Failing Tests
- Execute all available tests (unit, functional, integration).
- Identify failing tests and systematically debug and correct them.
- Confirm all tests pass post-fix.

### b. Fix Linting Issues
- Run appropriate linters for the project language (e.g., `clippy` for Rust, `eslint` for JavaScript).
- Resolve all warnings and errors, prioritizing idiomatic and efficient code solutions.
- Re-run linters to ensure clean results.

### c. Format Source Code
- Use standard formatting tools (e.g., `rustfmt`, `prettier`, `black`) to format all source files consistently.
- Ensure no formatting warnings or changes remain pending.

### d. Update Documentation
- Review affected documentation.
- Update comments, README files, and other documentation to reflect all code changes accurately.
- Include meaningful descriptions, usage examples, and doc tests as appropriate.

### e. Manage Git Status
- Run `git status` to identify changes.
- Categorize each file:
  - Stage for commit (`git add`) if relevant to the logical checkpoint.
  - Ignore/untrack irrelevant or temporary files via `.gitignore`.
  - Delete or move irrelevant files into an explicitly `.gitignored` folder when appropriate.
- Clearly communicate the rationale for each decision in the commit message or separate notes.

### f. Git Operations
- Stage changes with `git add`.
- Create a logical commit with a clear, descriptive message summarizing the checkpoint scope.
- Push the commit immediately (`git push`) to the remote repository, ensuring incremental backups and availability for testing and review on other systems.

---
