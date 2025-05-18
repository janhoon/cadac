use std::collections::HashSet;
use tree_sitter::{Node, Parser};

const NODE_KIND_SOURCE_FILE: &str = "source_file";
const NODE_KIND_SELECT_STATEMENT: &str = "select_statement";
const NODE_KIND_FROM_CLAUSE: &str = "from_clause";
const NODE_KIND_TABLE_REFERENCE: &str = "table_reference";
const NODE_KIND_TABLE_NAME: &str = "table_name";
const NODE_KIND_SCHEMA_NAME: &str = "schema_name";
const NODE_KIND_DATABASE_NAME: &str = "database_name";
const NODE_KIND_OBJECT_REFERENCE: &str = "object_reference";
const NODE_KIND_REFERENCE: &str = "reference";
const NODE_KIND_ALIAS: &str = "alias";
const NODE_KIND_JOIN: &str = "join";
const NODE_KIND_COLUMN_REFERENCE: &str = "column_reference";
const NODE_KIND_SELECT_LIST: &str = "select_list";
const NODE_KIND_SELECT_LIST_ITEM: &str = "select_list_item";

#[derive(Debug, PartialEq)]
pub enum ModelParseError {
    ParseError(String),
    MultipleStatements(usize),
}

impl std::fmt::Display for ModelParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelParseError::MultipleStatements(count) => write!(
                f,
                "Found {} SQL statements, but only 1 statement is allowed per model",
                count
            ),
            ModelParseError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ModelParseError {}

#[derive(Debug, PartialEq, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    // TODO: consider an enum here in the future
    pub data_type: Option<String>,
    // Names of sources that this column is extracted from, either source names or aliases as allowed by the SQL standard
    pub sources: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct ModelMetadata {
    pub name: String,
    pub description: Option<String>,
    pub columns: Vec<Column>,
    pub sources: Vec<Source>,
}

pub trait ModelParser {
    fn parse_model(&mut self, sql: &str) -> Result<&Self, ModelParseError>;
}

impl ModelParser for ModelMetadata {
    /// Parse the SQL string and extract the model metadata.
    fn parse_model(&mut self, sql: &str) -> Result<&Self, ModelParseError> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sql_cadac::LANGUAGE.into())
            .expect("Error loading sql grammar");
        let tree = parser.parse(sql, None).unwrap();
        let root_node = tree.root_node();
        let source_bytes = sql.as_bytes();

        match self.parse_root_node(root_node, source_bytes) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
}

impl ModelMetadata {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            columns: vec![],
            sources: vec![],
        }
    }

    /// Parse the root node of the tree and extract the select statement
    /// while making sure there is only one statement.
    fn parse_root_node(
        &mut self,
        node: tree_sitter::Node,
        source_bytes: &[u8],
    ) -> Result<(), ModelParseError> {
        assert_eq!(node.kind(), NODE_KIND_SOURCE_FILE);

        let mut statement_nodes = vec![];
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KIND_SELECT_STATEMENT {
                statement_nodes.push(child);
            }
        }

        if statement_nodes.len() > 1 {
            return Err(ModelParseError::MultipleStatements(statement_nodes.len()));
        }

        if node.has_error() {
            return Err(ModelParseError::ParseError("Error parsing SQL".to_string()));
        }

        if statement_nodes.is_empty() {
            return Err(ModelParseError::ParseError(
                "No SQL statements found".to_string(),
            ));
        }

        // Extract model description from the root node
        self.extract_model_description(&node, source_bytes);

        // Process the select statement to extract columns and sources
        self.walk_tree(statement_nodes[0], source_bytes);

        Ok(())
    }

    // Mutable reference to self for updating during parsing
    fn walk_tree(&mut self, n: Node, source_bytes: &[u8]) {
        // Create a cursor for tree traversal
        let mut cursor = n.walk();

        // Process current node
        self.process_node(&n, source_bytes);

        // Traverse children - first go to the first child
        if cursor.goto_first_child() {
            // Process the first child
            let child = cursor.node();
            self.walk_tree(child, source_bytes);

            // Process all siblings
            while cursor.goto_next_sibling() {
                let sibling = cursor.node();
                self.walk_tree(sibling, source_bytes);
            }
        }
    }

    fn process_node(&mut self, node: &Node, source_bytes: &[u8]) {
        // Get the current node kind
        let kind = node.kind();

        // Process node based on its kind
        match kind {
            // SQL statements
            NODE_KIND_SELECT_STATEMENT => {
                // Process select statement
                // This is handled by traversing its children
            }
            NODE_KIND_FROM_CLAUSE => {
                // Process from clause to extract source tables
                self.extract_sources_from_clause(node, source_bytes);
            }
            NODE_KIND_OBJECT_REFERENCE => {
                // Process object reference (table reference)
                if let Some(parent) = node.parent() {
                    if parent.kind() == NODE_KIND_FROM_CLAUSE {
                        self.extract_source_from_object_reference(node, source_bytes);
                    }
                }
            }
            NODE_KIND_TABLE_REFERENCE => {
                // Process table reference
                self.extract_source_from_table_reference(node, source_bytes);
            }
            NODE_KIND_SELECT_LIST => {
                // Process select list to extract columns
                self.extract_columns_from_select_list(node, source_bytes);
            }
            NODE_KIND_SELECT_LIST_ITEM => {
                // Process select list item to extract column
                self.extract_column_from_select_list_item(node, source_bytes);
            }
            NODE_KIND_JOIN => {
                // Process join to extract joined table as a source
                self.extract_source_from_join(node, source_bytes);
            }
            _ => {
                // Other node types are handled by traversing their children
            }
        }
    }

    // Extract model description from leading comments
    fn extract_model_description(&mut self, node: &Node, source_bytes: &[u8]) {
        // Look for comments at the beginning of the file
        let mut comments = Vec::new();

        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == "comment" {
                // Extract comment text
                for j in 0..child.child_count() {
                    let comment_part = child.child(j).unwrap();
                    if comment_part.kind() == "comment_text" {
                        let text = comment_part.utf8_text(source_bytes).unwrap_or("").trim();
                        comments.push(text.to_string());
                    }
                }
            } else if child.kind() == NODE_KIND_SELECT_STATEMENT {
                // Stop when we reach the SELECT statement
                break;
            }
        }

        // Join comments into a single description
        if !comments.is_empty() {
            self.description = Some(comments.join(" "));
        }
    }

    // Extract source tables from FROM clause
    fn extract_sources_from_clause(&mut self, node: &Node, source_bytes: &[u8]) {
        // Process all children of the FROM clause
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KIND_OBJECT_REFERENCE {
                self.extract_source_from_object_reference(&child, source_bytes);
            }
        }
    }

    // Extract source from object reference
    fn extract_source_from_object_reference(&mut self, node: &Node, source_bytes: &[u8]) {
        // Look for table reference within object reference
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KIND_TABLE_REFERENCE {
                self.extract_source_from_table_reference(&child, source_bytes);
            }
        }
    }

    // Extract source from table reference
    fn extract_source_from_table_reference(&mut self, node: &Node, source_bytes: &[u8]) {
        let mut talbe_name = String::new();
        let mut schema_name = String::new();
        let mut database_name = String::new();

        // Look for the name reference
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            // In tree-sitter-sql-cadac, the name field might be identified differently
            // We'll look for a reference node that's likely to be the name
            // if child.kind() == NODE_KIND_TABLE_NAME {
            //     let text = child.utf8_text(source_bytes).unwrap_or("").to_string();
            //     talbe_name = text;
            // }
            match child.kind() {
                NODE_KIND_TABLE_NAME => {
                    let text = child.utf8_text(source_bytes).unwrap_or("").to_string();
                    talbe_name = text;
                },
                NODE_KIND_SCHEMA_NAME => {
                    let text = child.utf8_text(source_bytes).unwrap_or("").to_string();
                    schema_name = text;
                },
                NODE_KIND_DATABASE_NAME => {
                    let text = child.utf8_text(source_bytes).unwrap_or("").to_string();
                    database_name = text;
                },
                _ => {}
            }
        }

        let fully_qualified_name = format!("{}.{}.{}", database_name, schema_name, talbe_name);

        for source in self.sources.iter() {
            if source.id == fully_qualified_name {
                break;
            }
        }
    }

    // Extract source from join
    fn extract_source_from_join(&mut self, node: &Node, source_bytes: &[u8]) {
        // Process join to find the joined table
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KIND_OBJECT_REFERENCE {
                self.extract_source_from_object_reference(&child, source_bytes);
            }
        }
    }

    // Extract columns from select list
    fn extract_columns_from_select_list(&mut self, node: &Node, source_bytes: &[u8]) {
        // Process all select list items
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KIND_SELECT_LIST_ITEM {
                self.extract_column_from_select_list_item(&child, source_bytes);
            } else if child.kind().contains("select_list_item") {
                // Handle other select list item types (like select_list_item_with_separator)
                self.extract_column_from_select_list_item(&child, source_bytes);
            }
        }
    }

    // Extract column from select list item
    fn extract_column_from_select_list_item(&mut self, node: &Node, source_bytes: &[u8]) {
        let mut column_name = String::new();
        let mut column_alias = String::new();
        let mut description = None;

        // Extract column reference
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();

            // Get column name from column reference
            if child.kind() == NODE_KIND_COLUMN_REFERENCE {
                for j in 0..child.child_count() {
                    let ref_child = child.child(j).unwrap();
                    if ref_child.kind() == NODE_KIND_REFERENCE {
                        column_name = ref_child.utf8_text(source_bytes).unwrap_or("").to_string();
                    }
                }
            }

            // Get alias if present
            if child.kind() == NODE_KIND_ALIAS {
                column_alias = child.utf8_text(source_bytes).unwrap_or("").to_string();
            }

            // Get description from comment
            if child.kind() == "comment" {
                for j in 0..child.child_count() {
                    let comment_part = child.child(j).unwrap();
                    if comment_part.kind() == "comment_text" {
                        let text = comment_part.utf8_text(source_bytes).unwrap_or("").trim();
                        description = Some(text.to_string());
                    }
                }
            }
        }

        // Use alias as column name if available
        let final_name = if !column_alias.is_empty() {
            column_alias
        } else {
            column_name
        };

        // Add column if we have a name
        if !final_name.is_empty() {
            let column = Column {
                name: final_name,
                description,
                data_type: None,     // We're not extracting data types yet
                sources: Vec::new(), // We're not tracking column sources yet
            };

            // Check if this column is already in the list
            if !self.columns.iter().any(|c| c.name == column.name) {
                self.columns.push(column);
            }
        }
    }
}
