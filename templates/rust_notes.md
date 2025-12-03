### Rust Development
- Use `cargo build` to compile the project
- Use `cargo test` to run all tests
- Use `cargo clippy --all-targets --all-features -- -D warnings` for strict linting
- Use `cargo fmt` for code formatting
- Use `cargo doc --open` to generate and view documentation
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Ensure all public items have documentation comments
- Use Rust 2024 edition features where applicable
- Prefer `let-else` patterns for error handling where appropriate
- Use workspace dependencies for multi-crate projects

### Rust/WASM Projects
- Keep JavaScript to absolute minimum - only for WASM loading
- All business logic must be in Rust
- Use `wasm-bindgen` for JS interop
- Use `wasm-pack` for building and packaging
- Write tests in Rust using `wasm-bindgen-test`, not in JavaScript
- Use `web-sys` for DOM manipulation from Rust
- Minimize JS bundle size - let Rust handle the complexity
