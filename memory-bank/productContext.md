# Product Context: CADAC

## Why This Project Exists
CADAC exists to address the challenges data teams face when working with data transformation, documentation, and sharing. Many organizations struggle with:

1. **Siloed Data Knowledge**: Information about data models, transformations, and sources is often scattered across different tools and teams.
2. **Documentation Gaps**: SQL transformations frequently lack proper documentation about their purpose, inputs, and outputs.
3. **Manual Cataloging**: Data assets are often manually cataloged, leading to outdated or incomplete information.
4. **Complex Toolchains**: Data teams use multiple tools for transformation, testing, and documentation, creating friction in workflows.

## Problems It Solves

### 1. Data Transformation Documentation
CADAC automatically extracts metadata from SQL queries, reducing the manual effort required to document transformations and making it easier to understand what each transformation does.

### 2. Data Cataloging
By parsing SQL and extracting information about data models, columns, and sources, CADAC builds a comprehensive catalog of data assets that can be easily searched and explored.

### 3. Knowledge Sharing
CADAC provides a centralized platform for data teams to share information about data assets, improving collaboration and reducing knowledge silos.

### 4. SQL Analysis
The tool analyzes SQL queries to extract meaningful information about data lineage, helping teams understand data flows and dependencies.

## How It Should Work

### User Experience Flow
1. **Input**: Users provide SQL queries or point to files containing SQL transformations
2. **Analysis**: CADAC parses the SQL using tree-sitter to extract metadata
3. **Cataloging**: The tool organizes the extracted information into a structured catalog
4. **Interaction**: Users interact with the catalog through a terminal UI
5. **Sharing**: Data assets and their metadata can be shared with other team members

### Key Features
- **SQL Parsing**: Extract metadata from SQL queries using tree-sitter
- **Model Metadata**: Capture information about data models, columns, and sources
- **Terminal UI**: Provide an intuitive interface for interacting with the catalog
- **Command-line Interface**: Enable scripting and automation through CLI commands
- **Data Lineage**: Track data flows between sources and destinations

### User Experience Goals
- **Simplicity**: Make it easy to catalog and document data transformations
- **Discoverability**: Enable users to quickly find relevant data assets
- **Integration**: Work seamlessly with existing data workflows and tools
- **Automation**: Reduce manual documentation and cataloging efforts
- **Clarity**: Provide clear insights into data models and their relationships
