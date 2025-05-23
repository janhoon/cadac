# Active Context: CADAC

## Current Work Focus
The project has completed its foundational components and is now ready to implement the dependency system:

1. **Dependency System Implementation**: Building intelligent dependency tracking for SQL models using pure SQL (no templating)
2. **Schema-Based Organization**: Implementing folder structure → schema mapping for natural database organization
3. **Graph-Based Dependency Resolution**: Integrating petgraph library for robust dependency analysis

## Recent Changes
- Completed SQL parser implementation with working metadata extraction
- Finished model discovery functionality with comprehensive testing
- All tests are now passing for parser and discovery components
- Established architectural decisions for dependency system design
- Decided on schema-based folder organization (models/schema/table.sql → schema.table)
- Selected petgraph library for dependency graph management
- Clarified pure SQL approach (no templating like dbt)

## Next Steps
1. **Implement Core Dependency System**
   - Add petgraph dependency to Cargo.toml
   - Create dependency data structures (ModelIdentity, DependencyGraph)
   - Implement schema-based model identity resolution from folder paths

2. **Smart Reference Resolution**
   - Enhance table reference parsing to distinguish qualified vs unqualified names
   - Implement resolution rules: schema.table → model lookup, db.schema.table → external
   - Add context-aware dependency matching based on current model's schema

3. **Graph Construction & Analysis**
   - Build dependency graph using petgraph with model qualified names as nodes
   - Implement cycle detection and topological sorting for execution order
   - Add dependency analysis methods (impact analysis, lineage tracking)

4. **Integration & Testing**
   - Extend ModelCatalog with dependency graph functionality
   - Create comprehensive tests for dependency resolution scenarios
   - Test with complex multi-schema model structures

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
