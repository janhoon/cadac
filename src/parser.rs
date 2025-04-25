use tree_sitter::Parser;

const NODE_KIND_SOURCE_FILE: &str = "source_file";
const NODE_KINE_SELECT_STATEMENT: &str = "select_statement";

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

trait ParseTreeSitter {
    fn parse_root_node(&self, node: tree_sitter::Node) -> Result<ModelMetadata, ModelParseError>;
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

impl ParseTreeSitter for ModelMetadata {
    fn parse_root_node(&self, node: tree_sitter::Node) -> Result<ModelMetadata, ModelParseError> {
        assert_eq!(node.kind(), NODE_KIND_SOURCE_FILE);

        let mut statement_nodes = vec![];
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == NODE_KINE_SELECT_STATEMENT {
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

        // Use the first statement node
        let _statements = statement_nodes[0];

        return Ok(ModelMetadata {
            name: self.name.clone(),
            description: None,
            columns: vec![],
            sources: vec![],
        });
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
                assert_eq!(*model, 
                    ModelMetadata {
                        name: "users".to_string(),
                        description: None,
                        columns: vec![
                            Column {
                                name: "a".to_string(),
                                description: None,
                                data_type: None,
                                sources: vec![],
                            },
                            Column {
                                name: "b".to_string(),
                                description: Some("test".to_string()),
                                data_type: None,
                                sources: vec![],
                            },
                            Column {
                                name: "c()".to_string(),
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
