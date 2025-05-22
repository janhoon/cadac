use tree_sitter::{Node, Parser};

const NODE_KIND_SOURCE_FILE: &str = "source_file";
const NODE_KIND_SELECT_STATEMENT: &str = "select_statement";
const NODE_KIND_FROM_CLAUSE: &str = "from_clause";
const NODE_KIND_TABLE_REFERENCE: &str = "table_reference";
const NODE_KIND_TABLE_NAME: &str = "table_name";
const NODE_KIND_SCHEMA_NAME: &str = "schema_name";
const NODE_KIND_DATABASE_NAME: &str = "database_name";
const NODE_KIND_OBJECT_REFERENCE: &str = "object_reference";
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

        // Extract model description from the select statement
        self.extract_model_description(&statement_nodes[0], source_bytes);

        // Process the select statement to extract columns and sources
        self.walk_tree(statement_nodes[0], source_bytes);

        Ok(())
    }

    // Mutable reference to self for updating during parsing
    fn walk_tree(&mut self, n: Node, source_bytes: &[u8]) {
        // Process current node and check if we should continue traversing
        let should_traverse_children = self.process_node(&n, source_bytes);
        
        if should_traverse_children {
            // Only traverse children if the node wasn't fully processed
            let mut cursor = n.walk();
            if cursor.goto_first_child() {
                let child = cursor.node();
                self.walk_tree(child, source_bytes);
                
                while cursor.goto_next_sibling() {
                    let sibling = cursor.node();
                    self.walk_tree(sibling, source_bytes);
                }
            }
        }
    }

    fn process_node(&mut self, node: &Node, source_bytes: &[u8]) -> bool {
        // Get the current node kind
        let kind = node.kind();

        // Process node based on its kind
        match kind {
            NODE_KIND_SELECT_LIST => {
                // Process all columns in the select list
                self.extract_columns_from_select_list(node, source_bytes);
                false // Don't traverse children - we handled them all
            }
            NODE_KIND_FROM_CLAUSE => {
                // Process all sources in the from clause
                self.extract_sources_from_clause(node, source_bytes);
                false // Don't traverse children - we handled them all
            }
            NODE_KIND_JOIN => {
                // Process join sources
                self.extract_source_from_join(node, source_bytes);
                false // Don't traverse children
            }
            _ => true // Continue traversing for other node types
        }
    }

    // Extract model description from comments within select_statement
    fn extract_model_description(&mut self, select_statement_node: &Node, source_bytes: &[u8]) {
        let mut comments = Vec::new();

        // Look for comment nodes that are direct children of select_statement
        for i in 0..select_statement_node.child_count() {
            let child = select_statement_node.child(i).unwrap();
            if child.kind() == "comment" {
                // Extract comment_text from the comment node
                if let Some(comment_text) = self.extract_comment_text(&child, source_bytes) {
                    comments.push(comment_text);
                }
            } else if child.kind() == "SELECT" {
                // Stop when we reach the SELECT keyword
                break;
            }
        }

        // Join comments into a single description
        if !comments.is_empty() {
            self.description = Some(comments.join(" "));
        }
    }

    // Helper function to extract comment_text from a comment node
    fn extract_comment_text(&self, comment_node: &Node, source_bytes: &[u8]) -> Option<String> {
        // Look for comment_text child node
        for i in 0..comment_node.child_count() {
            let child = comment_node.child(i).unwrap();
            if child.kind() == "comment_text" {
                let text = child.utf8_text(source_bytes).unwrap_or("").trim();
                if !text.is_empty() {
                    return Some(text.to_string());
                }
            }
        }
        None
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
        let mut table_name = String::new();
        let mut schema_name = String::new();
        let mut database_name = String::new();

        // Look for the name components
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            match child.kind() {
                NODE_KIND_TABLE_NAME => {
                    table_name = child.utf8_text(source_bytes).unwrap_or("").to_string();
                },
                NODE_KIND_SCHEMA_NAME => {
                    schema_name = child.utf8_text(source_bytes).unwrap_or("").to_string();
                },
                NODE_KIND_DATABASE_NAME => {
                    database_name = child.utf8_text(source_bytes).unwrap_or("").to_string();
                },
                _ => {}
            }
        }

        // For simple table names without schema/database, use the table name directly
        if table_name.is_empty() && schema_name.is_empty() && database_name.is_empty() {
            // Try to get the text content of the entire node as fallback
            table_name = node.utf8_text(source_bytes).unwrap_or("").to_string();
            // Remove any alias part (everything after "AS" or whitespace)
            if let Some(as_pos) = table_name.find(" AS ") {
                table_name = table_name[..as_pos].to_string();
            } else if let Some(space_pos) = table_name.find(' ') {
                table_name = table_name[..space_pos].to_string();
            }
        }

        if !table_name.is_empty() {
            let source_name = if !database_name.is_empty() && !schema_name.is_empty() {
                format!("{}.{}.{}", database_name, schema_name, table_name)
            } else if !schema_name.is_empty() {
                format!("{}.{}", schema_name, table_name)
            } else {
                table_name.clone()
            };

            // Check if this source already exists
            let mut found = false;
            if self.sources.iter().any(|s| s.id == source_name) {
                found = true;
            }

            if !found {
                let source = Source {
                    id: source_name.clone(),
                    name: table_name,
                    description: None,
                    database: if database_name.is_empty() { None } else { Some(database_name) },
                    schema: if schema_name.is_empty() { None } else { Some(schema_name) },
                };
                self.sources.push(source);
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
            }
        }
    }

    // Extract column from select list item based on actual tree structure
    fn extract_column_from_select_list_item(&mut self, node: &Node, source_bytes: &[u8]) {
        let mut column_name = String::new();
        let mut column_alias = String::new();
        let mut description = None;

        // Based on the tree structure:
        // select_list_item contains: comment, column_table_reference, column_reference, AS, alias, comment
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();

            match child.kind() {
                NODE_KIND_COLUMN_REFERENCE => {
                    // Get the column name directly from the column_reference node
                    column_name = child.utf8_text(source_bytes).unwrap_or("").to_string();
                },
                NODE_KIND_ALIAS => {
                    // Get the alias name
                    column_alias = child.utf8_text(source_bytes).unwrap_or("").to_string();
                },
                "comment" => {
                    // Extract description from comment using the helper function
                    if description.is_none() {
                        description = self.extract_comment_text(&child, source_bytes);
                    } else {
                        // If we already have a description, add the comment to it
                        description = Some(description.unwrap() + "\n" + &child.utf8_text(source_bytes).unwrap_or("").trim());
                    }
                },
                _ => {
                    // For other node types, check if they contain text that looks like a column name
                    if column_name.is_empty() {
                        let text = child.utf8_text(source_bytes).unwrap_or("").trim();
                        if !text.is_empty() && !text.contains(" ") && text != "AS" && text != "," {
                            column_name = text.to_string();
                        }
                    }
                }
            }
        }

        // Use alias as column name if available, otherwise use column name
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
