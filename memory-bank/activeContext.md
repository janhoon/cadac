# Active Context: CADAC

## Current Work Focus
The project has completed its foundational components and dependency system. Current focus areas:

1. **SQL Parsing Enhancements**: Improving table reference extraction and support for complex SQL constructs
2. **Smart Reference Resolution**: Building intelligent qualified vs unqualified table name resolution
3. **Terminal UI Development**: Beginning implementation of the interactive terminal interface

## Recent Changes
- ✅ Completed SQL parser implementation with working metadata extraction
- ✅ Finished model discovery functionality with comprehensive testing
- ✅ All tests are now passing for parser and discovery components
- ✅ Implemented complete dependency system with petgraph integration
- ✅ Created ModelIdentity structure for schema-based organization
- ✅ Built DependencyGraph with cycle detection and topological sorting
- ✅ Added execution order planning and dependency analysis methods
- ✅ Integrated dependency graph into ModelCatalog with comprehensive testing

## Next Steps
1. **SQL Parsing Enhancements (Priority)**
   - Enhance table reference extraction for dependency tracking
   - Support for complex SQL constructs (joins, CTEs, subqueries)
   - Improved handling of qualified column references
   - Better alias handling in dependency resolution

2. **Smart Reference Resolution**
   - Build intelligent qualified vs unqualified table name resolution
   - Implement context-aware dependency matching based on current model's schema
   - Handle database-qualified references as external tables

3. **Terminal UI Development**
   - Begin implementation of multi-view interface with ratatui
   - Create model browser view
   - Implement model detail view
   - Add dependency graph visualization

4. **CLI Commands Enhancement**
   - Command to run models in dependency order
   - Command to generate documentation
   - Command to query the catalog
   - Command to export catalog data

## Active Decisions and Considerations

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
