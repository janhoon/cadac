# Active Context: CADAC

## Current Work Focus
The project has completed its foundational SQL execution engine with PostgreSQL support and comprehensive integration testing. Current focus areas:

1. **Data Model Management System**: Implementing table refresh and incremental update capabilities for data models
2. **Model Execution Modes**: Building full refresh (CREATE TABLE AS) and incremental update strategies
3. **Terminal UI Development**: Beginning implementation of the interactive terminal interface
4. **SQL Parsing Enhancements**: Improving table reference extraction and support for complex SQL constructs
5. **Multi-Database Support (Future)**: Databricks and Snowflake adapters will be implemented after core features are stable

## Recent Changes
- ✅ Completed SQL parser implementation with working metadata extraction
- ✅ Finished model discovery functionality with comprehensive testing
- ✅ All tests are now passing for parser and discovery components (29 tests total)
- ✅ Implemented complete dependency system with petgraph integration
- ✅ Created ModelIdentity structure for schema-based organization
- ✅ Built DependencyGraph with cycle detection and topological sorting
- ✅ Added execution order planning and dependency analysis methods
- ✅ Integrated dependency graph into ModelCatalog with comprehensive testing
- ✅ Implemented complete SQL execution engine with async trait-based architecture
- ✅ Built fully functional PostgreSQL adapter with comprehensive connection management
- ✅ Added execution result tracking, status reporting, and connection validation
- ✅ Created optional feature flags for database adapters (postgres, databricks, snowflake)
- ✅ Added async-trait dependency and tokio-postgres for PostgreSQL support
- ✅ Implemented CLI run command with upstream/downstream dependencies, dry-run mode, fail-fast
- ✅ **COMPLETED: Comprehensive Integration Testing Enhancement**
  - Added 11 comprehensive integration test scenarios with test containers
  - Implemented transaction management and rollback capabilities
  - Enhanced error handling with PostgreSQL error categorization
  - Added execution metadata tracking (query hashing, timing, started_at)
  - Created graceful fallback handling for Docker container failures
  - Implemented comprehensive data transformation workflow testing (Bronze/Silver/Gold layers)

## Next Steps
1. **Data Model Management System (NEW PRIORITY)**
   - Implement table refresh modes: full refresh via CREATE TABLE AS
   - Add incremental update capabilities for data models
   - Abstract users from manual CREATE TABLE SQL commands
   - Build model execution strategies based on model configuration
   - Implement model materialization patterns (tables vs views)

2. **Model Execution Strategies**
   - Full refresh: DROP + CREATE TABLE AS SELECT pattern
   - Incremental updates: INSERT/UPDATE/DELETE based on change detection
   - Model dependency handling during refresh operations
   - Transaction management for model updates

3. **SQL Parsing Enhancements**
   - Enhance table reference extraction for dependency tracking
   - Support for complex SQL constructs (joins, CTEs, subqueries)
   - Improved handling of qualified column references
   - Better alias handling in dependency resolution

4. **Smart Reference Resolution**
   - Build intelligent qualified vs unqualified table name resolution
   - Implement context-aware dependency matching based on current model's schema
   - Handle database-qualified references as external tables

5. **Terminal UI Development**
   - Begin implementation of multi-view interface with ratatui
   - Create model browser view
   - Implement model detail view
   - Add dependency graph visualization

6. **Multi-Database Support (Future Phase)**
   - Implement Databricks adapter with proper SQL connector
   - Implement Snowflake adapter with proper SQL connector
   - Add comprehensive testing for all database adapters
   - Extend CLI to support multiple database targets

## Active Decisions and Considerations

### SQL Execution Architecture (Completed ✅)
- ✅ Multi-database support foundation: Postgres (fully implemented), Databricks (placeholder), Snowflake (placeholder)
- ✅ Platform-specific adapters using async trait-based architecture
- ✅ Connection management with connection string validation
- ✅ Execution result tracking with status, timing, and error reporting
- ✅ Optional feature flags for database-specific dependencies
- ✅ Model execution orchestration using dependency graph
- ✅ Upstream/downstream model selection for targeted execution
- ✅ Dry-run mode for execution planning and validation
- ✅ CLI run command with comprehensive execution options
- ✅ Comprehensive integration tests with test containers (11 test scenarios)
- ✅ Transaction management for rollback capabilities
- ✅ Comprehensive execution logging and monitoring
- ✅ Enhanced error handling and recovery with PostgreSQL error categorization

### Data Model Management Architecture (NEW FOCUS)
- 🔲 Model materialization strategies (full refresh vs incremental)
- 🔲 Table refresh via CREATE TABLE AS SELECT patterns
- 🔲 Incremental update detection and execution
- 🔲 Model configuration for execution modes
- 🔲 Abstraction layer over raw SQL commands for users
- 🔲 Model versioning and change detection
- 🔲 Dependency-aware model refresh orchestration

### Dependency System Architecture
- Using petgraph library for robust graph algorithms (cycle detection, topological sort)
- Schema-based folder organization: models/schema/table.sql → schema.table
- Pure SQL approach with no templating or special syntax
- Environment portability through connection string database resolution

### Reference Resolution Strategy
- Qualified references (schema.table) → direct model lookup
- Unqualified references (table) → search current schema first
- Database-qualified references (db.schema.table) → external tables, not model dependencies
- Context-aware resolution based on current model's schema location

### Model Identity System
- Folder structure determines schema: models/client/users.sql → client.users
- Database comes from connection string/environment, not folder structure
- Support for nested organization folders that don't affect qualification
- Qualified names (schema.table) as primary model identifiers

### Testing Strategy
- Implementing comprehensive unit tests for parser functionality
- Creating integration tests for model discovery
- Using tempfile for test file creation
- Following TDD principles for new feature development

## Important Patterns and Preferences

### Code Organization
- Modules organized by functionality (parser, discovery, cli)
- Clear separation between parsing, discovery, and UI components
- Using traits for interface definitions (e.g., ModelParser)
- Implementing builder pattern for ModelMetadata construction

### Error Handling
- Using color-eyre for rich error reporting
- Implementing std::error::Error for custom error types
- Providing detailed error messages with context
- Using the ? operator for error propagation

### Testing Approach
- Writing tests before implementation (TDD)
- Creating specific test cases for different SQL constructs
- Testing error conditions and edge cases
- Using temporary directories for file-based tests

## Learnings and Project Insights

### SQL Parsing Challenges
- Tree-sitter grammar has specific node types that need careful handling
- SQL dialects have variations in syntax and semantics
- Comments contain valuable metadata that needs extraction
- AST traversal requires understanding of the grammar structure

### Model Discovery Insights
- File system traversal needs robust error handling
- Model naming conventions affect discovery logic
- Building a catalog requires efficient data structures
- Dependency tracking needs careful consideration of SQL semantics

### Project Evolution
- Moving from basic parsing to more sophisticated metadata extraction
- Shifting focus to model relationships and dependencies
- Building toward a comprehensive data catalog
- Preparing for terminal UI development
