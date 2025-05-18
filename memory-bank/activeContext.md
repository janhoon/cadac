# Active Context: CADAC

## Current Work Focus
The project is in early development stages, focusing on establishing the core architecture and functionality:

1. **SQL Parsing Infrastructure**: Building the foundation for parsing SQL queries using tree-sitter
2. **Model Metadata Extraction**: Developing structures and logic to extract metadata from SQL
3. **Basic Terminal UI**: Setting up a simple terminal UI using ratatui

## Recent Changes
- Initial project setup with core dependencies
- Implementation of basic SQL parsing functionality
- Creation of model metadata structures
- Setup of a minimal terminal UI

## Next Steps
1. **Complete SQL Parser Implementation**
   - Implement full traversal of SQL AST
   - Extract column metadata including descriptions from comments
   - Handle various SQL statement types

2. **Enhance Model Metadata**
   - Add support for data types
   - Implement source tracking
   - Add validation for metadata

3. **Develop Terminal UI**
   - Create views for browsing models
   - Implement navigation between views
   - Add search functionality

4. **Implement CLI Commands**
   - Add command for parsing SQL files
   - Add command for generating documentation
   - Add command for querying the catalog

## Active Decisions and Considerations

### Parser Implementation
- Currently using a visitor pattern for tree traversal
- Considering whether to extract more metadata from SQL comments
- Evaluating how to handle different SQL dialects

### Terminal UI Design
- Deciding on the layout and navigation flow
- Considering how to display model relationships
- Evaluating performance for large catalogs

### Project Structure
- Evaluating whether to split functionality into more modules
- Considering whether to add more abstraction layers
- Deciding on the organization of tests

## Important Patterns and Preferences

### Code Organization
- Modules are organized by functionality (args, cli, parser)
- Core data structures are defined in the parser module
- CLI interface is separated from business logic

### Error Handling
- Using color-eyre for rich error reporting
- Defining custom error types for domain-specific errors
- Propagating errors with the `?` operator

### Testing Approach
- Unit tests for parser functionality
- Test cases with sample SQL queries
- Error case testing

## Learnings and Project Insights

### SQL Parsing Challenges
- SQL has many dialects and variations
- Comments can contain valuable metadata
- Tree traversal needs careful handling of node types

### Terminal UI Considerations
- Terminal UIs have limited space and interaction capabilities
- Need to balance information density with usability
- Event handling requires careful state management

### Project Evolution
- Starting with core parsing functionality before expanding to more features
- Focusing on robustness and correctness before optimization
- Building a solid foundation for future enhancements
