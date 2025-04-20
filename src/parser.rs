use tree_sitter::Parser;

pub fn parse_sql(sql: &str) {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_sql::LANGUAGE.into())
        .expect("Error loading sql grammar");
    let tree = parser.parse(sql, None).unwrap();
    let root_node = tree.root_node();
    println!("{:?}", root_node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sql() {
        let sql = "SELECT a, b, c, as b FROM users";
        parse_sql(sql);
    }
}
