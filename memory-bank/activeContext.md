# Active Context: CADAC

## Current Work Focus
The project is in early development stages, with significant progress on the core SQL parsing and model discovery functionality:

1. **SQL Parsing Infrastructure**: Implementing robust SQL parsing using tree-sitter with focus on metadata extraction
2. **Model Discovery**: Building functionality to discover and catalog SQL models from files
3. **Test-Driven Development**: Developing comprehensive tests for parser and discovery components

## Recent Changes
- Enhanced SQL parser implementation with improved AST traversal
- Added support for extracting column metadata from SELECT statements
- Implemented model discovery functionality to find and process SQL files
- Created test suite for parser and discovery components
- Added support for handling select_list_item_with_separator nodes
- Improved error handling in the parser with std::error::Error implementation

## Next Steps
1. **Fix Current Test Failures**
   - Address issues with source table extraction in the parser
   - Fix column metadata extraction in SELECT statements
   - Ensure model descriptions are correctly parsed from comments

2. **Complete SQL Parser Implementation**
   - Finish implementation of source table extraction
   - Improve handling of table aliases and qualified column names
   - Support more complex SQL constructs (joins, CTEs)

3. **Enhance Model Discovery**
   - Implement dependency tracking between models
   - Build dependency graph based on source/target relationships
   - Add validation for discovered models

4. **Develop Terminal UI**
   - Create views for browsing discovered models
   - Implement visualization of model dependencies
   - Add search and filtering capabilities

## Active Decisions and Considerations

### Parser Implementation
- Using a recursive tree traversal approach for AST processing
- Extracting metadata from SQL comments for documentation
- Handling different node types in the tree-sitter SQL grammar
- Considering how to best represent source tables and their relationships to columns

### Model Discovery
- Using file system traversal to find SQL files
- Extracting model names from filenames
- Building a catalog of models with their metadata
- Planning for dependency graph construction

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
