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
   - Repository: https://github.com/janhoon/tree-sitter-sql

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

5. **tempfile**: Temporary file and directory creation
   - Purpose: Create temporary files and directories for testing
   - Version: 3.20.0
   - Used in: Test suite for model discovery

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
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Core library functionality
│   ├── args.rs          # Command-line argument definitions
│   ├── cli.rs           # CLI implementation
│   ├── parser.rs        # SQL parsing logic
│   ├── discovery.rs     # Model discovery functionality
│   ├── parser_test.rs   # Tests for parser
│   └── discovery_test.rs # Tests for discovery
├── Cargo.toml           # Project manifest
├── Cargo.lock           # Dependency lock file
└── README.md            # Project documentation
```

### Build Process
1. Clone the repository
2. Run `cargo build` to compile the project
3. Run `cargo test` to execute tests
4. Run `cargo run` to execute the application

## Technical Constraints

### Performance Considerations
- SQL parsing must be efficient for large queries
- Model discovery should handle large directories with many SQL files
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
- Safe file system operations during model discovery

## Dependencies

### Direct Dependencies
- tree-sitter: SQL parsing
- tree-sitter-sql-cadac: Custom SQL grammar
- ratatui: Terminal UI
- crossterm: Terminal manipulation
- clap: Command-line argument parsing
- color-eyre: Error handling
- tempfile: Temporary file creation for tests

### Development Dependencies
- cc: Native code compilation

## Tool Usage Patterns

### SQL Parsing
```rust
// Example of parsing SQL with tree-sitter and ModelParser trait
let mut model = ModelMetadata::new("model_name".to_string());
match model.parse_model(sql) {
    Ok(model) => {
        // Use the populated model metadata
        println!("Model name: {}", model.name);
        println!("Columns: {:?}", model.columns);
        println!("Sources: {:?}", model.sources);
    },
    Err(err) => {
        // Handle parsing error
        eprintln!("Error parsing SQL: {}", err);
    }
}
```

### Model Discovery
```rust
// Example of discovering models from a directory
let mut catalog = ModelCatalog::new(PathBuf::from("models/"));
match catalog.discover_models() {
    Ok(()) => {
        // Use the discovered models
        println!("Discovered {} models", catalog.models.len());
        for (name, model) in &catalog.models {
            println!("Model: {}", name);
        }
    },
    Err(err) => {
        // Handle discovery error
        eprintln!("Error discovering models: {}", err);
    }
}
```

### Terminal UI (Planned)
```rust
// Example of terminal UI setup with ratatui
let terminal = ratatui::init();
terminal.draw(|f| {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());
    
    // Render model list
    let models = catalog.models.keys().collect::<Vec<_>>();
    let model_list = List::new(models)
        .block(Block::default().title("Models").borders(Borders::ALL));
    f.render_widget(model_list, chunks[0]);
    
    // Render model details
    if let Some(selected_model) = selected_model {
        let model = &catalog.models[selected_model];
        let details = Paragraph::new(format!("Name: {}\nColumns: {}\nSources: {}", 
            model.name, model.columns.len(), model.sources.len()))
            .block(Block::default().title("Details").borders(Borders::ALL));
        f.render_widget(details, chunks[1]);
    }
})?;
```

### Command-line Arguments
```rust
// Example of command-line argument definition with clap
#[derive(Parser, Debug)]
pub struct RunCmdArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
    
    #[arg(short, long)]
    run_model: Option<String>,
    
    #[arg(short, long)]
    recursive: bool,
}
```

### Error Handling
```rust
// Example of error handling with color-eyre
color_eyre::install()?;

// Function that returns Result<T, E>
fn process_sql_file(file_path: &Path) -> Result<()> {
    let sql_content = fs::read_to_string(file_path)
        .wrap_err_with(|| format!("Failed to read SQL file: {:?}", file_path))?;
    
    let mut model = ModelMetadata::new(file_path.file_stem()?.to_str()?.to_string());
    model.parse_model(&sql_content)
        .map_err(|e| eyre!("Failed to parse model: {}", e))?;
    
    Ok(())
}
```

### Testing
```rust
// Example of test with temporary files
#[test]
fn test_discover_models() -> Result<()> {
    // Create a temporary directory for test models
    let temp_dir = tempdir()?;
    let model_dir = temp_dir.path().to_path_buf();

    // Create test SQL files
    create_test_sql_file(&model_dir, "model1.sql", "SELECT a, b FROM source1")?;
    
    // Test model discovery
    let mut catalog = ModelCatalog::new(model_dir);
    catalog.discover_models()?;
    
    // Verify results
    assert_eq!(catalog.models.len(), 1);
    assert!(catalog.models.contains_key("model1"));
    
    Ok(())
}
```
