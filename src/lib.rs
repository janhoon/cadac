use cli::main_cli;
use parser::{ModelMetadata, ModelParser};

mod args;
mod cli;
mod discovery;
mod parser;

#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod discovery_test;

pub fn exec_cadac() {
    // let sql = parse_sql("SELECT a, b, c FROM users");
    // println!("{:?}", sql.name);
    // main_cli().unwrap();
    main_cli().unwrap();
    let mut model = ModelMetadata::new("users".to_string());
    let sql = "SELECT a, b, c from d";
    let _ = model.parse_model(sql);
}
