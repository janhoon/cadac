# Progress: CADAC

## Current Status
CADAC is in early development (version 0.1.0) with completed foundational components. The project has successfully implemented SQL parsing with tree-sitter and model discovery functionality, with all tests passing. The focus has now shifted to implementing the dependency system using petgraph for robust graph algorithms and schema-based folder organization.

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

### Dependency System (Priority)
- 🔲 Add petgraph dependency to project
- 🔲 Create ModelIdentity structure for schema-based organization
- 🔲 Implement DependencyGraph with petgraph integration
- 🔲 Build smart reference resolution (qualified vs unqualified table names)
- 🔲 Add cycle detection and topological sorting
- 🔲 Implement execution order planning

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

### CLI Commands
- 🔲 Command to run models
- 🔲 Command to generate documentation
- 🔲 Command to query the catalog
- 🔲 Command to export catalog data

## Known Issues
1. No dependency tracking between models (next priority)
2. Schema-based folder organization not implemented
3. Terminal UI is not yet implemented
4. No execution order planning
5. Limited support for complex SQL constructs
6. No cycle detection for model dependencies

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
- 🔄 Model discovery with dependency tracking
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
