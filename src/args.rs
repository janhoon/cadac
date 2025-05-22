use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct BaseCliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parse a single SQL file and display metadata
    Parse {
        /// Path to the SQL file to parse
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Discover and parse all SQL files in a directory
    Discover {
        /// Directory containing SQL model files
        #[arg(short, long, default_value = "models/")]
        model_path: PathBuf,
    },
    /// Launch the terminal UI
    Tui,
}

#[derive(Parser, Debug)]
pub struct RunCmdArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
}
