use args::{BaseCliArgs, Commands};
use clap::Parser;
use cli::main_cli;
use color_eyre::Result;
use discovery::ModelCatalog;
use parser::{ModelMetadata, ModelParser};
use std::fs;

mod args;
mod cli;
mod dependency_graph;
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

        let source_names: Vec<String> = model.sources.iter().map(|s| s.id.clone()).collect();
        println!("   📊 Sources: {}", source_names.join(", "));

        let column_names: Vec<String> = model.columns.iter().map(|c| c.name.clone()).collect();
        println!("   📋 Columns: {}", column_names.join(", "));
    }

    // Build and display dependency graph
    println!("\n🔗 Building dependency graph...");
    catalog.build_dependency_graph()?;

    println!("📊 Dependency Graph:");
    println!("   Models: {}", catalog.dependency_graph.model_count());
    println!("   Dependencies: {}", catalog.dependency_graph.dependency_count());

    // Check for circular dependencies
    if catalog.has_circular_dependencies() {
        println!("   ⚠️  Circular dependencies detected!");
    } else {
        println!("   ✅ No circular dependencies");
    }

    // Show execution order
    match catalog.get_execution_order() {
        Ok(order) => {
            println!("\n🚀 Execution Order:");
            for (i, model) in order.iter().enumerate() {
                println!("   {}. {}", i + 1, model);
            }
        }
        Err(e) => {
            println!("\n❌ Cannot determine execution order: {}", e);
        }
    }

    // Show dependencies for each model
    println!("\n🔍 Model Dependencies:");
    for model_name in catalog.models.keys() {
        let dependencies = catalog.get_dependencies(model_name);
        let dependents = catalog.get_dependents(model_name);
        
        println!("   📄 {}", model_name);
        if !dependencies.is_empty() {
            println!("      ⬅️  Depends on: {}", dependencies.join(", "));
        }
        if !dependents.is_empty() {
            println!("      ➡️  Used by: {}", dependents.join(", "));
        }
        if dependencies.is_empty() && dependents.is_empty() {
            println!("      🔸 No internal dependencies");
        }
    }

    Ok(())
}
