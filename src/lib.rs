mod parser;

use parser::parse_sql;

pub fn print_my_name() {
    parse_sql("SELECT a, b, c FROM users");
}

