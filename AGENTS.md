# Agent Guidelines for CADAC

## Build/Test Commands
- Build: `cargo build`
- Test all: `cargo test`
- Test single: `cargo test <test_name>`
- Run: `cargo run -- <subcommand>`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Code Style
- **Language**: Rust (2024 edition)
- **Imports**: Group std, external crates, then local modules with blank lines between
- **Naming**: snake_case for functions/variables, PascalCase for types/enums, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use `color_eyre::Result<()>` for main functions, custom error enums for domain errors
- **Types**: Prefer explicit types, use `#[derive(Debug, PartialEq, Clone)]` for data structures
- **Documentation**: Use `///` for public APIs, `//` for inline comments
- **Testing**: Tests in separate `*_test.rs` files with `#[cfg(test)]` module declaration

## Memory Bank Integration
- This project uses Avante's Memory Bank system (see `memory.planning.avanterules`)
- Always read memory-bank files before starting work: `projectbrief.md`, `productContext.md`, `activeContext.md`, `systemPatterns.md`, `techContext.md`, `progress.md`
- Update memory bank after significant changes or when requested with "update memory bank"

## Project Structure
- CLI tool for SQL model parsing and dependency analysis
- Uses tree-sitter for SQL parsing, clap for CLI, ratatui for TUI
- Main modules: args, cli, parser, discovery, dependency_graph