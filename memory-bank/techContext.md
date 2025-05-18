# Technical Context: CADAC

## Technologies Used

### Programming Language
- **Rust**: Modern systems programming language with memory safety guarantees
  - Version: Edition 2024
  - Benefits: Performance, safety, robust type system, excellent package ecosystem

### Core Libraries
1. **tree-sitter**: Incremental parsing library
   - Purpose: Parse SQL queries into abstract syntax trees
   - Version: 0.25.3
   - Custom SQL grammar: tree-sitter-sql-cadac (v0.1.5)

2. **ratatui**: Terminal UI framework
   - Purpose: Create interactive terminal user interfaces
   - Version: 0.29.0
   - Dependencies: crossterm (0.29.0) for terminal manipulation

3. **clap**: Command-line argument parser
   - Purpose: Process command-line arguments and options
   - Version: 4.5.37
   - Features: derive (for declarative argument definitions)

4. **color-eyre**: Error handling and reporting
   - Purpose: Provide rich, colorful error reports
   - Version: 0.6.3

### Build System
- **Cargo**: Rust's package manager and build system
  - Build dependencies: cc (for native code compilation)

## Development Setup

### Environment Requirements
- Rust toolchain (rustc, cargo)
- Git (for version control and dependency management)

### Project Structure
```
cadac/
├── src/
│   ├── main.rs       # Entry point
│   ├── lib.rs        # Core library functionality
│   ├── args.rs       # Command-line argument definitions
│   ├── cli.rs        # CLI implementation
│   └── parser.rs     # SQL parsing logic
├── Cargo.toml        # Project manifest
├── Cargo.lock        # Dependency lock file
└── README.md         # Project documentation
```

### Build Process
1. Clone the repository
2. Run `cargo build` to compile the project
3. Run `cargo test` to execute tests
4. Run `cargo run` to execute the application

## Technical Constraints

### Performance Considerations
- SQL parsing must be efficient for large queries
- Terminal UI should be responsive and low-latency
- Memory usage should be optimized for handling large data catalogs

### Compatibility Requirements
- Cross-platform support (Linux, macOS, Windows)
- Support for various SQL dialects (currently focused on standard SQL)
- Terminal compatibility (supports modern terminal emulators)

### Security Considerations
- Safe handling of user-provided SQL queries
- Proper error handling to prevent crashes or undefined behavior
- No execution of SQL queries (parsing only)

## Dependencies

### Direct Dependencies
- tree-sitter: SQL parsing
- tree-sitter-sql-cadac: Custom SQL grammar
- ratatui: Terminal UI
- crossterm: Terminal manipulation
- clap: Command-line argument parsing
- color-eyre: Error handling

### Development Dependencies
- cc: Native code compilation

## Tool Usage Patterns

### SQL Parsing
```rust
// Example of parsing SQL with tree-sitter
let mut parser = Parser::new();
parser.set_language(&tree_sitter_sql_cadac::LANGUAGE.into())?;
let tree = parser.parse(sql, None).unwrap();
let root_node = tree.root_node();
```

### Terminal UI
```rust
// Example of terminal UI setup with ratatui
let terminal = ratatui::init();
terminal.draw(render)?;
if matches!(event::read()?, Event::Key(_)) {
    // Handle key event
}
ratatui::restore();
```

### Command-line Arguments
```rust
// Example of command-line argument definition with clap
#[derive(Parser, Debug)]
pub struct RunCmdArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
}
```

### Error Handling
```rust
// Example of error handling with color-eyre
color_eyre::install()?;
// Function that returns Result<T, E>
let result = run(terminal);
// Error will be formatted with color-eyre
```
