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
- Run `clippy` (for Rust projects) or equivalent linters.
- Resolve all warnings and errors, prioritizing idiomatic and efficient code solutions.
- Re-run linters to ensure clean results.

### c. Format Source Code
- Use standard formatting tools (e.g., `rustfmt`, `prettier`) to format all source files consistently.
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

## 2. Quality-Oriented Development

Ensure all generated code adheres strictly to the following quality standards:

### Documentation
- Clearly document all code modules, functions, and key logic blocks.
- Ensure compatibility with automated documentation generation tools (`cargo doc`, `Sphinx`, etc.).
- Write and maintain accurate, relevant doc tests within code comments to demonstrate correct usage.

### Testing
- Include unit tests for individual functions, validating core logic and handling edge cases.
- Provide functional tests verifying code behavior meets defined requirements and user stories.
- Integrate code modules through comprehensive integration tests confirming interoperability and end-to-end correctness.
- Explicitly defer more advanced testing (stress, load, benchmarks, fuzz testing) until explicitly requested by the developer.

---

## 3. Continuous Improvement

Actively integrate learning from previous mistakes into the proactive development cycle to prevent recurring errors:

### Build and Test Failures
- Document all encountered build and test failures with specific causes.
- Regularly review failure logs and integrate solutions into the standard development workflow proactively.
- Maintain a growing reference of common issues and their resolutions for quick reference.

### Clippy Warnings and Lint Errors
- Track common clippy warnings and linting issues encountered historically.
- Proactively avoid patterns that have caused past warnings, incorporating this knowledge into the code generation and review phases.

### Process Documentation Updates
- Immediately update this document whenever you encounter and solve recurring build, test, or lint issues.
- Clearly specify new guidelines or modifications to existing ones to eliminate similar future problems.
- Use precise examples from historical failures to illustrate improved practices clearly.

---

## Summary of Expected Proactive Behaviors:

- Always anticipate checkpoints and proactively align code with defined processes.
- Prioritize incremental, clean commits with immediate pushes to maintain transparency and availability.
- Document thoroughly, test rigorously, format consistently, and manage version control meticulously.
- Continuously adapt and integrate historical lessons into proactive workflow adjustments to minimize repetitive errors.

By diligently following these guidelines, AI coding agents will significantly enhance development efficiency, maintainability, and overall code quality.

---

Copyright (c) 2025 Michael A Wright. Licensed under MIT License.
