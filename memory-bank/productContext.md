# Product Context: CADAC

## Why This Project Exists
CADAC exists to address the challenges data teams face when working with data transformation, documentation, and sharing. Many organizations struggle with:

1. **Siloed Data Knowledge**: Information about data models, transformations, and sources is often scattered across different tools and teams.
2. **Documentation Gaps**: SQL transformations frequently lack proper documentation about their purpose, inputs, and outputs.
3. **Manual Cataloging**: Data assets are often manually cataloged, leading to outdated or incomplete information.
4. **Complex Toolchains**: Data teams use multiple tools for transformation, testing, and documentation, creating friction in workflows.
5. **Dependency Management**: Understanding and managing dependencies between data models is challenging and error-prone.

## Problems It Solves

### 1. Data Transformation Documentation
CADAC automatically extracts metadata from SQL queries, reducing the manual effort required to document transformations and making it easier to understand what each transformation does.

### 2. Data Cataloging
By parsing SQL and extracting information about data models, columns, and sources, CADAC builds a comprehensive catalog of data assets that can be easily searched and explored.

### 3. Knowledge Sharing
CADAC provides a centralized platform for data teams to share information about data assets, improving collaboration and reducing knowledge silos.

### 4. SQL Analysis
The tool analyzes SQL queries to extract meaningful information about data lineage, helping teams understand data flows and dependencies.

### 5. Dependency Management
CADAC automatically identifies dependencies between models by analyzing SQL queries, enabling proper sequencing of transformations and impact analysis.

## How It Should Work

### User Experience Flow
1. **Discovery**: CADAC scans directories to find SQL model files ✅
2. **Analysis**: The tool parses SQL using tree-sitter to extract metadata ✅
3. **Dependency Mapping**: Dependencies between models are identified and mapped ✅
4. **Cataloging**: Extracted information is organized into a structured catalog ✅
5. **Execution**: Models can be run in the correct dependency order 🔄
6. **Visualization**: Users interact with the catalog through a terminal UI 🔲

### Key Features
- **SQL Parsing**: Extract metadata from SQL queries using tree-sitter ✅
- **Model Discovery**: Automatically find and catalog SQL models from directories ✅
- **Model Metadata**: Capture information about data models, columns, and sources ✅
- **Dependency Tracking**: Identify and manage relationships between models ✅
- **SQL Execution**: Execute models in correct dependency order across multiple database platforms 🔄
- **Multi-Database Support**: Support for Postgres (✅), Databricks (🔲 future), and Snowflake (🔲 future)
- **Upstream/Downstream Execution**: Run models with their dependencies or dependents 🔲
- **Terminal UI**: Provide an intuitive interface for interacting with the catalog 🔲
- **Command-line Interface**: Enable scripting and automation through CLI commands 🔄

### User Experience Goals
- **Simplicity**: Make it easy to catalog and document data transformations
- **Discoverability**: Enable users to quickly find relevant data assets
- **Integration**: Work seamlessly with existing data workflows and tools
- **Automation**: Reduce manual documentation and cataloging efforts
- **Clarity**: Provide clear insights into data models and their relationships
- **Efficiency**: Run models in the correct order with minimal user intervention

### Comparison with dbt
CADAC is designed as an alternative to dbt (data build tool) with some key differences:

1. **SQL-First Approach**: Unlike dbt which uses Jinja templating, CADAC works directly with pure SQL files
2. **Tree-sitter Parsing**: Uses a robust parsing approach instead of regex-based extraction
3. **Terminal UI**: Focuses on a terminal-based interface rather than web UI
4. **Lightweight**: Designed to be a lightweight tool with minimal dependencies
5. **Rust Implementation**: Built for performance and reliability using Rust
