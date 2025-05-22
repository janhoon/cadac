use args::{BaseCliArgs, Commands};
use clap::Parser;
use cli::main_cli;
use color_eyre::Result;
use discovery::ModelCatalog;
use parser::{ModelMetadata, ModelParser};
use std::fs;

mod args;
mod cli;
mod discovery;
mod parser;

#[cfg(test)]
mod discovery_test;
#[cfg(test)]
mod parser_test;

pub fn exec_cadac() {
    if let Err(e) = run_cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_cli() -> Result<()> {
    color_eyre::install()?;

    let args = BaseCliArgs::parse();

    match args.command {
        Commands::Parse { file } => {
            parse_sql_file(file)?;
        }
        Commands::Discover { model_path } => {
            discover_models(model_path)?;
        }
        Commands::Tui => {
            main_cli()?;
        }
    }

    Ok(())
}

fn parse_sql_file(file_path: std::path::PathBuf) -> Result<()> {
    // Read the SQL file
    let sql_content = fs::read_to_string(&file_path)?;

    // Extract model name from filename
    let model_name = file_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Parse the SQL
    let mut model = ModelMetadata::new(model_name.clone());
    model.parse_model(&sql_content)?;

    // Display the results
    println!("📄 Model: {}", model.name);

    if let Some(description) = &model.description {
        println!("📝 Description: {}", description);
    }

    println!("\n📊 Sources ({}):", model.sources.len());
    for source in &model.sources {
        println!("  • {} ({})", source.name, source.id);
        if let Some(desc) = &source.description {
            println!("    {}", desc);
        }
    }

    println!("\n📋 Columns ({}):", model.columns.len());
    for column in &model.columns {
        print!("  • {}", column.name);
        if let Some(desc) = &column.description {
            print!(" - {}", desc);
        }
        println!();
    }

    Ok(())
}

fn discover_models(model_path: std::path::PathBuf) -> Result<()> {
    println!("🔍 Discovering models in: {}", model_path.display());

    let mut catalog = ModelCatalog::new(model_path);
    catalog.discover_models()?;

    println!("📚 Found {} models:", catalog.models.len());

    for (name, model) in &catalog.models {
        println!("\n📄 {}", name);

        if let Some(description) = &model.description {
            println!("   📝 {}", description);
        }

        let source_names: Vec<String> = model.sources.iter().map(|s| s.name.clone()).collect();
        println!("   📊 Sources: {}", source_names.join(", "));

        let column_names: Vec<String> = model.columns.iter().map(|c| c.name.clone()).collect();
        println!("   📋 Columns: {}", column_names.join(", "));
    }

    Ok(())
}
