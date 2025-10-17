<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Rust Workspace: msk_musik

This workspace contains a Rust project with a library named `musik_theory`. The project follows standard Rust workspace conventions.

## Project Structure

- Root workspace contains `Cargo.toml` with workspace configuration
- `musik_theory/` - Core music library crate
- Standard Rust development patterns and conventions

## GitHub Configuration

- user.name: veminovici
- Repository URL: https://github.com/veminovici/msk_musik
- Use this username for all GitHub-related links and references

## Development Guidelines

- Follow Rust naming conventions (snake_case for modules, functions, variables)
- Use `cargo` commands for building, testing, and running
- Implement proper error handling with Result types
- Write comprehensive tests for library functions
- Document public APIs with rustdoc comments
- Use constants as much as possible instead of hardcoded values
- Avoid unnecessary allocations and clones
- Prefer iterators and functional programming constructs over loops
- Use pattern matching and enums for better code clarity
- Leverage Rust's ownership model to manage resources effectively
- Minimize the use of `unwrap` and `expect` in favor of proper error handling
- Use `Result` and `Option` types for functions that can fail or return nothing
- Use const functions where applicable

## Workspace Commands

- `cargo build` - Build all workspace members
- `cargo test` - Run tests for all workspace members
- `cargo check` - Check code without building
- `cargo fmt` - Format code according to Rust standards
- `cargo clippy` - Run lints for better code quality
