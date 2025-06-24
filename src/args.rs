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
    /// Run models with dependency resolution
    Run {
        /// Directory containing SQL model files
        #[arg(short, long, default_value = "models/")]
        model_path: PathBuf,
        /// Specific model to run (if not specified, runs all models)
        #[arg(short = 'n', long)]
        model_name: Option<String>,
        /// Include upstream dependencies
        #[arg(short = 'u', long)]
        upstream: bool,
        /// Include downstream dependents
        #[arg(short = 'd', long)]
        downstream: bool,
        /// Dry run (show execution plan without running)
        #[arg(long)]
        dry_run: bool,
        /// Fail fast on first error
        #[arg(long)]
        fail_fast: bool,
        /// Database connection string
        #[arg(short = 'c', long)]
        connection: String,
    },
}

#[derive(Parser, Debug)]
pub struct RunCmdArgs {
    #[arg(short, long, default_value = "models/")]
    model_path: String,
}
