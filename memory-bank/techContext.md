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

2. **petgraph**: Graph data structure library
   - Purpose: Dependency graph management with robust algorithms
   - Version: 0.8.1 (implemented)
   - Features: Cycle detection, topological sorting, graph traversal
   - Used for: Model dependency tracking and execution order planning

3. **Database Drivers** (Planned)
   - **tokio-postgres**: Async PostgreSQL driver
   - **databricks-sql-connector**: Databricks SQL connector
   - **snowflake-connector**: Snowflake database connector
   - Purpose: Multi-database support for SQL execution

4. **ratatui**: Terminal UI framework
   - Purpose: Create interactive terminal user interfaces
   - Version: 0.29.0
   - Dependencies: crossterm (0.29.0) for terminal manipulation

5. **clap**: Command-line argument parser
   - Purpose: Process command-line arguments and options
   - Version: 4.5.37
   - Features: derive (for declarative argument definitions)

6. **color-eyre**: Error handling and reporting
   - Purpose: Provide rich, colorful error reports
   - Version: 0.6.3

7. **tempfile**: Temporary file and directory creation
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
│   ├── dependency_graph.rs # Dependency graph implementation
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
- Secure SQL execution with proper connection management
- Safe file system operations during model discovery
- Database credential management and secure connection handling

## Dependencies

### Direct Dependencies
- tree-sitter: SQL parsing
- tree-sitter-sql-cadac: Custom SQL grammar
- petgraph: Dependency graph management
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

### Dependency System (Implemented)
```rust
// Example of dependency graph usage with petgraph
use petgraph::{Graph, Direction};
use petgraph::algo::{is_cyclic_directed, toposort};

pub struct DependencyGraph {
    graph: Graph<String, ()>,  // Node = model qualified name, Edge = dependency
    node_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn add_dependency(&mut self, from_model: &str, to_model: &str) -> Result<()> {
        let from_idx = self.get_or_create_node(from_model);
        let to_idx = self.get_or_create_node(to_model);
        self.graph.add_edge(to_idx, from_idx, ()); // to_model → from_model (dependency direction)
        Ok(())
    }
    
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }
    
    pub fn execution_order(&self) -> Result<Vec<String>> {
        toposort(&self.graph, None)
            .map(|nodes| nodes.iter().map(|&i| self.graph[i].clone()).collect())
            .map_err(|_| eyre!("Circular dependency detected"))
    }
}

// Example of schema-based model identity
pub struct ModelIdentity {
    pub file_path: PathBuf,           // models/client/users.sql
    pub table_name: String,           // users
    pub schema_name: String,          // client
    pub qualified_name: String,       // client.users
}

impl ModelIdentity {
    pub fn from_path(file_path: PathBuf, models_root: &Path) -> Result<Self> {
        let relative_path = file_path.strip_prefix(models_root)?;
        let schema_name = relative_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre!("Cannot extract schema from path"))?;
        let table_name = file_path.file_stem()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre!("Cannot extract table name from path"))?;
        
        Ok(ModelIdentity {
            file_path,
            table_name: table_name.to_string(),
            schema_name: schema_name.to_string(),
            qualified_name: format!("{}.{}", schema_name, table_name),
        })
    }
}
```

### SQL Execution Engine (Planned)
```rust
// Example of SQL execution with multi-database support
use cadac::execution::{DatabaseAdapter, ExecutionEngine, ExecutionPlan};

// Database adapter trait for different platforms
pub trait DatabaseAdapter {
    async fn connect(&self, connection_string: &str) -> Result<Box<dyn Connection>>;
    async fn execute_sql(&self, connection: &dyn Connection, sql: &str) -> Result<ExecutionResult>;
    fn dialect(&self) -> SqlDialect;
}

// Execution engine for orchestrating model runs
pub struct ExecutionEngine {
    adapters: HashMap<DatabaseType, Box<dyn DatabaseAdapter>>,
    catalog: ModelCatalog,
}

impl ExecutionEngine {
    pub async fn run_model(&self, model_name: &str, options: RunOptions) -> Result<ExecutionResult> {
        // Get execution plan based on dependencies
        let plan = self.create_execution_plan(model_name, &options)?;
        
        // Execute models in dependency order
        for model in plan.execution_order {
            let sql = self.catalog.get_model_sql(&model)?;
            let adapter = self.get_adapter_for_model(&model)?;
            
            match adapter.execute_sql(&connection, &sql).await {
                Ok(result) => {
                    println!("✅ Model {} executed successfully", model);
                    self.log_execution_success(&model, &result);
                },
                Err(err) => {
                    eprintln!("❌ Model {} failed: {}", model, err);
                    if options.fail_fast {
                        return Err(err);
                    }
                }
            }
        }
        
        Ok(ExecutionResult::Success)
    }
    
    pub fn create_execution_plan(&self, model_name: &str, options: &RunOptions) -> Result<ExecutionPlan> {
        let mut models_to_run = vec![model_name.to_string()];
        
        // Add upstream dependencies if requested
        if options.include_upstream {
            let upstream = self.catalog.get_dependencies(model_name);
            models_to_run.extend(upstream);
        }
        
        // Add downstream dependents if requested
        if options.include_downstream {
            let downstream = self.catalog.get_dependents(model_name);
            models_to_run.extend(downstream);
        }
        
        // Get execution order respecting dependencies
        let execution_order = self.catalog.get_execution_order_for_models(&models_to_run)?;
        
        Ok(ExecutionPlan {
            models: models_to_run,
            execution_order,
            dry_run: options.dry_run,
        })
    }
}

// Run options for model execution
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub include_upstream: bool,
    pub include_downstream: bool,
    pub dry_run: bool,
    pub fail_fast: bool,
    pub target_database: Option<String>,
}

// CLI command for running models
#[derive(Parser, Debug)]
pub struct RunCommand {
    /// Model name to run
    model_name: String,
    
    /// Include upstream dependencies
    #[arg(short = 'u', long)]
    upstream: bool,
    
    /// Include downstream dependents
    #[arg(short = 'd', long)]
    downstream: bool,
    
    /// Dry run (show execution plan without running)
    #[arg(long)]
    dry_run: bool,
    
    /// Fail fast on first error
    #[arg(long)]
    fail_fast: bool,
    
    /// Target database connection
    #[arg(short = 't', long)]
    target: Option<String>,
}

// Database-specific adapters
pub struct PostgresAdapter {
    pool: tokio_postgres::Pool,
}

impl DatabaseAdapter for PostgresAdapter {
    async fn connect(&self, connection_string: &str) -> Result<Box<dyn Connection>> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        
        // Spawn connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        
        Ok(Box::new(PostgresConnection { client }))
    }
    
    async fn execute_sql(&self, connection: &dyn Connection, sql: &str) -> Result<ExecutionResult> {
        let pg_conn = connection.as_any().downcast_ref::<PostgresConnection>()
            .ok_or_else(|| eyre!("Invalid connection type for Postgres adapter"))?;
        
        let rows = pg_conn.client.execute(sql, &[]).await?;
        
        Ok(ExecutionResult {
            rows_affected: rows,
            execution_time: std::time::Duration::from_millis(0), // TODO: measure time
            status: ExecutionStatus::Success,
        })
    }
    
    fn dialect(&self) -> SqlDialect {
        SqlDialect::Postgres
    }
}

// Similar adapters for Databricks and Snowflake...
pub struct DatabricksAdapter { /* ... */ }
pub struct SnowflakeAdapter { /* ... */ }
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
