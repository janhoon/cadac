# Progress: CADAC

## Current Status
CADAC is in early development (version 0.1.0) with completed foundational components and dependency system. The project has successfully implemented SQL parsing with tree-sitter, model discovery functionality, and dependency tracking using petgraph, with all tests passing. The focus has now shifted to implementing SQL execution capabilities across multiple database platforms (Postgres, Databricks, Snowflake) and enhancing the terminal UI.

## What Works

### SQL Parsing
- ✅ Tree-sitter integration for SQL parsing
- ✅ Model metadata structure definition
- ✅ SQL statement validation
- ✅ Error handling with std::error::Error implementation
- ✅ Detection of multiple statements
- ✅ Basic column extraction from SELECT statements
- ✅ Comment parsing for descriptions
- ✅ Support for column aliases

### Model Discovery
- ✅ File system traversal to find SQL files
- ✅ SQL file content reading
- ✅ Model metadata extraction from SQL files
- ✅ Model catalog structure
- ✅ Error handling for file operations
- ✅ Support for recursive directory traversal
- ✅ Schema-based model identity extraction from folder paths
- ✅ Dependency graph construction and integration

### Testing Framework
- ✅ Unit tests for parser functionality
- ✅ Integration tests for model discovery
- ✅ Test utilities for creating temporary files
- ✅ Error case testing

### Command-line Interface
- ✅ Basic CLI structure with clap
- ✅ Command-line argument definitions
- ✅ Error handling with color-eyre

## What's Left to Build

### Dependency System (Completed ✅)
- ✅ Add petgraph dependency to project
- ✅ Create ModelIdentity structure for schema-based organization
- ✅ Implement DependencyGraph with petgraph integration
- ✅ Add cycle detection and topological sorting
- ✅ Implement execution order planning
- 🔲 Build smart reference resolution (qualified vs unqualified table names)

### Schema-Based Model Organization
- 🔲 Update ModelCatalog to support schema.table naming
- 🔲 Extract schema from folder structure (models/schema/table.sql)
- 🔲 Implement context-aware dependency resolution
- 🔲 Support for nested organization folders
- 🔲 Handle database-qualified references as external tables

### SQL Parsing Enhancements
- 🔲 Enhanced table reference extraction for dependency tracking
- 🔲 Support for complex SQL constructs (joins, CTEs, subqueries)
- 🔲 Improved handling of qualified column references
- 🔲 Support for data types
- 🔲 Better alias handling in dependency resolution

### Terminal UI Development
- 🔲 Multi-view interface with ratatui
- 🔲 Model browser view
- 🔲 Model detail view
- 🔲 Dependency graph visualization
- 🔲 Search functionality
- 🔲 Keyboard shortcuts
- 🔲 Help documentation

### Data Catalog
- 🔲 Persistent storage for catalog data
- 🔲 Data lineage visualization
- 🔲 Metadata search capabilities
- 🔲 Export functionality

### SQL Execution Engine (New Priority)
- 🔲 Database connection management (Postgres, Databricks, Snowflake)
- 🔲 SQL execution engine with platform-specific adapters
- 🔲 Model execution in dependency order
- 🔲 Upstream/downstream model selection and execution
- 🔲 Transaction management and rollback capabilities
- 🔲 Execution status tracking and logging
- 🔲 Error handling for failed model executions
- 🔲 Dry-run mode for execution planning

### CLI Commands
- 🔲 Command to run models (with upstream/downstream options)
- 🔲 Command to generate documentation
- 🔲 Command to query the catalog
- 🔲 Command to export catalog data

## Known Issues
1. Smart reference resolution for qualified vs unqualified table names needs enhancement
2. Terminal UI is not yet implemented
3. Limited support for complex SQL constructs
4. Context-aware dependency resolution needs improvement

## Evolution of Project Decisions

### Parser Implementation
- **Initial Decision**: Use tree-sitter for SQL parsing
- **Current Status**: Basic parsing works, but metadata extraction needs improvement
- **Future Direction**: Enhance AST traversal and metadata extraction, fix current issues

### Model Discovery
- **Initial Decision**: Implement file-based model discovery
- **Current Status**: Basic discovery works, but dependency tracking is missing
- **Future Direction**: Add dependency graph construction and validation

### Testing Approach
- **Initial Decision**: Use test-driven development
- **Current Status**: Comprehensive test suite with some failing tests
- **Future Direction**: Fix failing tests and continue TDD for new features

### Project Structure
- **Initial Decision**: Organize by functionality (parser, discovery, cli)
- **Current Status**: Clear separation of concerns with dedicated modules
- **Future Direction**: Add UI module and potentially split parser into submodules

## Milestones

### Milestone 1: Foundation (Completed)
- ✅ Project setup
- ✅ Basic SQL parsing
- ✅ Core data structures
- ✅ Model discovery framework

### Milestone 2: Core Functionality (Current)
- 🔄 Complete SQL parser with metadata extraction
- ✅ Model discovery with dependency tracking
- 🔲 Basic terminal UI
- 🔲 CLI commands for basic operations

### Milestone 3: Enhanced Features
- 🔲 Dependency graph visualization
- 🔲 Model execution
- 🔲 Advanced terminal UI
- 🔲 Documentation generation

### Milestone 4: Production Readiness
- 🔲 Comprehensive error handling
- 🔲 Performance optimization
- 🔲 User documentation
- 🔲 Packaging and distribution
