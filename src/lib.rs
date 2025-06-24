use args::{BaseCliArgs, Commands};
use clap::Parser;
use cli::main_cli;
use color_eyre::Result;
use discovery::ModelCatalog;
use execution::{create_engine_with_available_adapters, SqlDialect};
use parser::{ModelMetadata, ModelParser};
use std::fs;

mod args;
mod cli;
mod dependency_graph;
mod discovery;
mod execution;
mod parser;

#[cfg(test)]
mod discovery_test;
#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod execution_test;

pub fn exec_cadac() {
    #[cfg(any(feature = "postgres", feature = "databricks", feature = "snowflake"))]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Err(e) = rt.block_on(run_cli()) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    
    #[cfg(not(any(feature = "postgres", feature = "databricks", feature = "snowflake")))]
    {
        if let Err(e) = run_cli_sync() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(any(feature = "postgres", feature = "databricks", feature = "snowflake"))]
async fn run_cli() -> Result<()> {
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
        Commands::Run {
            model_path,
            model_name,
            upstream,
            downstream,
            dry_run,
            fail_fast,
            connection,
        } => {
            #[cfg(any(feature = "postgres", feature = "databricks", feature = "snowflake"))]
            {
                run_models(
                    model_path,
                    model_name,
                    upstream,
                    downstream,
                    dry_run,
                    fail_fast,
                    connection,
                ).await?;
            }
            
            #[cfg(not(any(feature = "postgres", feature = "databricks", feature = "snowflake")))]
            {
                return Err(color_eyre::eyre::eyre!(
                    "No database adapters available. Please install CADAC with database support:\n\
                    - For PostgreSQL: cargo install cadac --features postgres\n\
                    - For all databases: cargo install cadac --features all-databases"
                ));
            }
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
    println!(
        "   Dependencies: {}",
        catalog.dependency_graph.dependency_count()
    );

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

#[cfg(not(any(feature = "postgres", feature = "databricks", feature = "snowflake")))]
fn run_cli_sync() -> Result<()> {
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
        Commands::Run { .. } => {
            return Err(color_eyre::eyre::eyre!(
                "No database adapters available. Please install CADAC with database support:\n\
                - For PostgreSQL: cargo install cadac --features postgres\n\
                - For all databases: cargo install cadac --features all-databases"
            ));
        }
    }

    Ok(())
}

#[cfg(any(feature = "postgres", feature = "databricks", feature = "snowflake"))]
async fn run_models(
    model_path: std::path::PathBuf,
    model_name: Option<String>,
    upstream: bool,
    downstream: bool,
    dry_run: bool,
    fail_fast: bool,
    connection: String,
) -> Result<()> {
    println!("🚀 Running models from: {}", model_path.display());
    
    // Create execution engine with available adapters
    let engine = create_engine_with_available_adapters();
    
    // Check if any database adapters are available
    let available_dialects = engine.available_dialects();
    if available_dialects.is_empty() {
        return Err(color_eyre::eyre::eyre!(
            "No database adapters available. Please install CADAC with database support:\n\
            - For PostgreSQL: cargo install cadac --features postgres\n\
            - For all databases: cargo install cadac --features all-databases"
        ));
    }
    
    println!("📊 Available database adapters: {:?}", available_dialects);
    
    // Discover models and build dependency graph
    let mut catalog = ModelCatalog::new(model_path);
    catalog.discover_models()?;
    catalog.build_dependency_graph()?;
    
    println!("📚 Found {} models", catalog.models.len());
    
    // Check for circular dependencies
    if catalog.has_circular_dependencies() {
        return Err(color_eyre::eyre::eyre!("Circular dependencies detected! Cannot execute models."));
    }
    
    // Determine which models to run
    let models_to_run = if let Some(specific_model) = model_name {
        let mut models = vec![specific_model.clone()];
        
        if upstream {
            let deps = catalog.get_dependencies(&specific_model);
            models.extend(deps);
        }
        
        if downstream {
            let dependents = catalog.get_dependents(&specific_model);
            models.extend(dependents);
        }
        
        models.sort();
        models.dedup();
        models
    } else {
        // Run all models
        catalog.models.keys().cloned().collect()
    };
    
    // Get execution order
    let execution_order = catalog.get_execution_order()?;
    let filtered_execution_order: Vec<String> = execution_order
        .into_iter()
        .filter(|model| models_to_run.contains(model))
        .collect();
    
    println!("\n📋 Execution Plan:");
    for (i, model) in filtered_execution_order.iter().enumerate() {
        println!("  {}. {}", i + 1, model);
    }
    
    if dry_run {
        println!("\n🔍 Dry run completed. No models were executed.");
        return Ok(());
    }
    
    // Determine dialect from connection string
    let dialect = if connection.starts_with("postgresql://") || connection.starts_with("postgres://") {
        SqlDialect::Postgres
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Cannot determine database dialect from connection string. Supported prefixes:\n\
            - PostgreSQL: postgresql:// or postgres://"
        ));
    };
    
    // Check if the required dialect is supported
    if !engine.supports_dialect(&dialect) {
        return Err(color_eyre::eyre::eyre!(
            "Database dialect {:?} is not supported. Available dialects: {:?}\n\
            Install CADAC with the appropriate feature flag to enable support.",
            dialect, available_dialects
        ));
    }
    
    // Execute models
    let mut success_count = 0;
    let mut failed_count = 0;
    
    println!("\n🔄 Executing models...");
    
    for model_name in &filtered_execution_order {
        if let Some(model_identity) = catalog.model_identities.get(model_name) {
            println!("\n📄 Executing: {}", model_name);
            
            // Read the SQL file content
            let sql_content = std::fs::read_to_string(&model_identity.file_path)?;
            
            match engine.execute_sql(&sql_content, &connection, dialect.clone()).await {
                Ok(result) => {
                    match result.status {
                        execution::ExecutionStatus::Success => {
                            println!("  ✅ Success - {} rows affected in {:?}", 
                                result.rows_affected, result.execution_time);
                            success_count += 1;
                        }
                        execution::ExecutionStatus::Failed => {
                            println!("  ❌ Failed - {}", 
                                result.message.unwrap_or_else(|| "Unknown error".to_string()));
                            failed_count += 1;
                            if fail_fast {
                                return Err(color_eyre::eyre::eyre!("Model execution failed: {}", model_name));
                            }
                        }
                        execution::ExecutionStatus::Skipped => {
                            println!("  ⏭️  Skipped");
                        }
                    }
                }
                Err(e) => {
                    println!("  ❌ Error: {}", e);
                    failed_count += 1;
                    if fail_fast {
                        return Err(e);
                    }
                }
            }
        }
    }
    
    println!("\n📊 Execution Summary:");
    println!("  ✅ Successful: {}", success_count);
    println!("  ❌ Failed: {}", failed_count);
    println!("  📋 Total: {}", filtered_execution_order.len());
    
    if failed_count > 0 {
        return Err(color_eyre::eyre::eyre!("{} model(s) failed to execute", failed_count));
    }
    
    Ok(())
}
