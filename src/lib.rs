mod args;
mod cli;
mod parser;

use args::CadacCliArgs;
use clap::Parser;
use cli::main_cli;
use parser::parse_sql;

pub fn exec_cadac() {
    let sql = parse_sql("SELECT a, b, c FROM users");
    println!("{:?}", sql.name);
    CadacCliArgs::parse();
    main_cli().unwrap();
}
