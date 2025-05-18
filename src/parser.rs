use tree_sitter::{Node, Parser};

const NODE_KIND_SOURCE_FILE: &str = "source_file";
const NODE_KIND_SELECT_STATEMENT: &str = "select_statement";

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

#[derive(Debug, PartialEq)]
pub struct Source {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub description: Option<String>,
    // TODO: consider an enum here in the future
    pub data_type: Option<String>,
    pub sources: Vec<Source>,
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
        let _ = match self.parse_root_node(root_node) {
            Ok(model) => model,
            Err(err) => return Err(err),
        };

        return Ok(self);
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
    fn parse_root_node(&self, node: tree_sitter::Node) -> Result<ModelMetadata, ModelParseError> {
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

        self.walk_tree(statement_nodes[0]);

        return Ok(ModelMetadata {
            name: self.name.clone(),
            description: None,
            columns: vec![],
            sources: vec![],
        });
    }

    fn walk_tree(&self, n: Node) {
        // Create a cursor for tree traversal
        let mut cursor = n.walk();

        // Process current node
        self.process_node(&n);

        // Traverse children - first go to the first child
        if cursor.goto_first_child() {
            // Process the first child
            let child = cursor.node();
            self.walk_tree(child);

            // Process all siblings
            while cursor.goto_next_sibling() {
                let sibling = cursor.node();
                self.walk_tree(sibling);
            }
        }
    }

    fn process_node(&self, node: &Node) {
        // Get the current node kind
        let kind = node.kind();

        println!("{}", kind);
        // Process node based on its kind
        match kind {
            // SQL statements
            "source_file" => {
                // Call function for source_file nodes
                // process_source_file(node);
            }
            "select_statement" => {
                // Call function for select_statement nodes
                // process_select_statement(node);
            }
            "from_clause" => {
                // Call function for from_clause nodes
                // process_from_clause(node);
            }
            "where_clause" => {
                // Call function for where_clause nodes
                // process_where_clause(node);
            }
            "group_by_clause" => {
                // Call function for group_by_clause nodes
                // process_group_by_clause(node);
            }
            "having_clause" => {
                // Call function for having_clause nodes
                // process_having_clause(node);
            }
            "order_by_clause" => {
                // Call function for order_by_clause nodes
                // process_order_by_clause(node);
            }

            // Table references
            "table_reference" => {
                // Call function for table_reference nodes
                // process_table_reference(node);
            }
            "object_reference" => {
                // Call function for object_reference nodes
                // process_object_reference(node);
            }

            // Column references
            "column_reference" => {
                // Call function for column_reference nodes
                // process_column_reference(node);
            }
            "select_list" => {
                // Call function for select_list nodes
                // process_select_list(node);
            }
            "select_list_item" => {
                // Call function for select_list_item nodes
                // process_select_list_item(node);
            }
            "select_list_item_with_separator" => {
                // Call function for select_list_item_with_separator nodes
                // process_select_list_item_with_separator(node);
            }

            // Comments and metadata
            "comment" => {
                // Call function for comment nodes
                // process_comment(node);
            }
            "comment_text" => {
                // Call function for comment_text nodes
                // process_comment_text(node);
            }

            // Functions and expressions
            "function_call" => {
                // Call function for function_call nodes
                // process_function_call(node);
            }
            "expression" => {
                // Call function for expression nodes
                // process_expression(node);
            }

            // Literals
            "string_literal" => {
                // Call function for string_literal nodes
                // process_string_literal(node);
            }
            "number_literal" => {
                // Call function for number_literal nodes
                // process_number_literal(node);
            }

            // Joins
            "join" => {
                // Call function for join nodes
                // process_join(node);
            }

            // References
            "reference" => {
                // Call function for reference nodes
                // process_reference(node);
            }

            // Any other node type
            _ => {
                // Call default handler for unspecified node types
                // process_default_node(node);
            }
        }
    }

    /*
    Select statement example:

    -- comment

    -- comment
    -- comment
    select
    col1 as col1_alias, -- col1 description
    col2,
    -- col3 description
    col3 -- col3 description
    from db.schema.table1 AS t;

    Tree sitter tree:

    (source_file [0, 0] - [11, 0]
        (select_statement [0, 0] - [9, 26]
            (comment [0, 0] - [0, 10]
            (COMMENT [0, 0] - [0, 2])
            (comment_text [0, 2] - [0, 10]))
            (comment [2, 0] - [2, 10]
            (COMMENT [2, 0] - [2, 2])
            (comment_text [2, 2] - [2, 10]))
            (comment [3, 0] - [3, 10]
            (COMMENT [3, 0] - [3, 2])
            (comment_text [3, 2] - [3, 10]))
            (SELECT [4, 0] - [4, 6])
            (select_list [5, 0] - [9, 0]
            (select_list_item_with_separator [5, 0] - [6, 0]
                expression: (column_reference [5, 0] - [5, 4]
                column_ref: (reference [5, 0] - [5, 4]))
                (AS [5, 5] - [5, 7])
                alias: (reference [5, 8] - [5, 18])
                inline_comment: (comment [5, 20] - [5, 39]
                (COMMENT [5, 20] - [5, 22])
                (comment_text [5, 22] - [5, 39])))
            (select_list_item_with_separator [6, 0] - [7, 0]
                expression: (column_reference [6, 0] - [6, 4]
                column_ref: (reference [6, 0] - [6, 4])))
            (select_list_item [7, 0] - [9, 0]
                comment: (comment [7, 0] - [7, 19]
                (COMMENT [7, 0] - [7, 2])
                (comment_text [7, 2] - [7, 19]))
                expression: (column_reference [8, 0] - [8, 4]
                column_ref: (reference [8, 0] - [8, 4]))
                inline_comment: (comment [8, 5] - [8, 24]
                (COMMENT [8, 5] - [8, 7])
                (comment_text [8, 7] - [8, 24]))))
            (from_clause [9, 0] - [9, 26]
            (FROM [9, 0] - [9, 4])
            (object_reference [9, 5] - [9, 26]
                (table_reference [9, 5] - [9, 26]
                database: (reference [9, 5] - [9, 7])
                schema: (reference [9, 8] - [9, 14])
                name: (reference [9, 15] - [9, 21])
                (AS [9, 22] - [9, 24])
                alias: (reference [9, 25] - [9, 26]))))))
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model() {
        let sql = "
        -- this is the model description
        SELECT
            a,
            b as test,
            c() as \"test2\"
        FROM users;";
        let name: String = "users".to_string();
        let mut model = ModelMetadata::new(name);
        let result = model.parse_model(sql);
        match result {
            Ok(model) => {
                assert_eq!(model.name, "users");
                assert_eq!(
                    *model,
                    ModelMetadata {
                        name: "users".to_string(),
                        description: Some("this it the model description".to_string()),
                        columns: vec![],
                        sources: vec![],
                    }
                )
            }
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }

    #[test]
    fn test_should_return_error_when_multiple_statements() {
        let sql = "
        SELECT
            a,
            b,
            c
        FROM users;

        SELECT
            a,
            b,
            c
        FROM users";
        let name: String = "users".to_string();
        let mut model = ModelMetadata::new(name);
        let result = model.parse_model(sql);
        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(err, ModelParseError::MultipleStatements(2)),
        }
    }
}
