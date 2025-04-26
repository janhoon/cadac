use tree_sitter::Parser;

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

pub trait ParseModel {
    fn parse_model(&mut self, sql: &str) -> Result<&Self, ModelParseError>;
}

impl ModelMetadata {
    fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            columns: vec![],
            sources: vec![],
        }
    }
}

impl ParseModel for ModelMetadata {
    /// Parse the SQL string and extract the model metadata.
    fn parse_model(&mut self, sql: &str) -> Result<&Self, ModelParseError> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sql::LANGUAGE.into())
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

        let statement = statement_nodes[0];

        return Ok(ModelMetadata {
            name: self.name.clone(),
            description: None,
            columns: vec![],
            sources: vec![],
        });
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

    /// Parse a select statement node
    /// and extract the columns and their descriptions.
    fn parse_select_statement(
        &self,
        node: tree_sitter::Node,
    ) -> Result<ModelMetadata, ModelParseError> {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model() {
        let sql = "
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
                        columns: vec![
                            Column {
                                name: "a".to_string(),
                                description: None,
                                data_type: None,
                                sources: vec![],
                            },
                            Column {
                                name: "test".to_string(),
                                description: Some("test".to_string()),
                                data_type: None,
                                sources: vec![],
                            },
                            Column {
                                name: "test2".to_string(),
                                description: Some("test2".to_string()),
                                data_type: None,
                                sources: vec![],
                            },
                        ],
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
