## 2. Quality-Oriented Development

Ensure all generated code adheres strictly to the following quality standards:

### Documentation
- Clearly document all code modules, functions, and key logic blocks.
- Ensure compatibility with automated documentation generation tools (`cargo doc`, `Sphinx`, `JSDoc`, etc.).
- Write and maintain accurate, relevant doc tests within code comments to demonstrate correct usage.

### Testing
- Include unit tests for individual functions, validating core logic and handling edge cases.
- Provide functional tests verifying code behavior meets defined requirements and user stories.
- Integrate code modules through comprehensive integration tests confirming interoperability and end-to-end correctness.
- For Rust/WASM projects: Write tests in Rust, not Python. Use `wasm-bindgen-test` for WASM-specific tests.
- Explicitly defer more advanced testing (stress, load, benchmarks, fuzz testing) until explicitly requested by the developer.

### Tech Debt Avoidance

**File Size Limits**:
- Keep source files under 500 lines (prefer 200-300 lines)
- Split large files into logical modules
- If a file exceeds 500 lines, refactor immediately

**TODO Comments**:
- Limit to 3 TODO comments per file maximum
- Address TODOs within 2 development sessions
- Convert persistent TODOs to GitHub issues
- Never commit code with FIXMEs - resolve immediately

**Code Complexity**:
- Functions should be under 50 lines (prefer 10-30)
- Cyclomatic complexity should stay below 10
- Deeply nested code (>3 levels) indicates need for refactoring

**Dependencies**:
- Audit dependencies regularly - remove unused ones
- Prefer well-maintained, popular crates/packages
- Minimize dependency tree depth

### Scripting Guidelines

**Bash Scripts**:
- Use bash for simple automation (< 100 lines)
- Use shellcheck for validation
- Always use `set -euo pipefail` at the start
- Quote all variables: `"${var}"` not `$var`

**When to Switch to Python**:
- Script exceeds 100 lines
- Complex data structures needed
- JSON/YAML parsing required
- Cross-platform compatibility needed
- Error handling becomes complex

**Python Scripts**:
- Use type hints even in scripts
- Include `#!/usr/bin/env python3` shebang
- Use `argparse` for CLI arguments
- Keep scripts focused - one purpose per script

---
