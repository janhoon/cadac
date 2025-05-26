use color_eyre::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

use crate::discovery::ModelCatalog;

#[test]
fn test_discover_models_in_directory() -> Result<()> {
    // Create a temporary directory for test models
    let temp_dir = tempdir()?;
    let model_dir = temp_dir.path().to_path_buf();

    // Create schema directories
    let bronze_dir = model_dir.join("bronze");
    let silver_dir = model_dir.join("silver");
    fs::create_dir(&bronze_dir)?;
    fs::create_dir(&silver_dir)?;

    // Create test SQL files with schema-based organization
    create_test_sql_file(&bronze_dir, "users.sql", "SELECT a, b FROM source1")?;
    create_test_sql_file(&bronze_dir, "orders.sql", "SELECT c, d FROM source2")?;
    create_test_sql_file(&silver_dir, "customers.sql", "SELECT e, f FROM bronze.users")?;

    // Create a non-SQL file that should be ignored
    let non_sql_path = bronze_dir.join("not_a_model.txt");
    let mut non_sql_file = File::create(non_sql_path)?;
    writeln!(non_sql_file, "This is not a SQL file")?;

    // Discover models
    let mut catalog = ModelCatalog::new(model_dir);
    catalog.discover_models()?;

    // Verify that all SQL models were discovered with qualified names
    assert_eq!(catalog.models.len(), 3);
    assert!(catalog.models.contains_key("bronze.users"));
    assert!(catalog.models.contains_key("bronze.orders"));
    assert!(catalog.models.contains_key("silver.customers"));

    // Verify that non-SQL files were ignored
    assert!(!catalog.models.contains_key("not_a_model"));

    // Verify model identities were created
    assert_eq!(catalog.model_identities.len(), 3);
    assert!(catalog.model_identities.contains_key("bronze.users"));
    assert!(catalog.model_identities.contains_key("bronze.orders"));
    assert!(catalog.model_identities.contains_key("silver.customers"));

    // Verify model content
    let bronze_users = catalog.models.get("bronze.users").unwrap();
    assert_eq!(bronze_users.sources.len(), 1);
    assert_eq!(bronze_users.sources.iter().next().unwrap().name, "source1");

    let bronze_orders = catalog.models.get("bronze.orders").unwrap();
    assert_eq!(bronze_orders.sources.len(), 1);
    assert_eq!(bronze_orders.sources.iter().next().unwrap().name, "source2");

    let silver_customers = catalog.models.get("silver.customers").unwrap();
    assert_eq!(silver_customers.sources.len(), 1);
    assert_eq!(silver_customers.sources.iter().next().unwrap().id, "bronze.users");

    // Verify model identities
    let bronze_users_identity = catalog.model_identities.get("bronze.users").unwrap();
    assert_eq!(bronze_users_identity.schema_name, "bronze");
    assert_eq!(bronze_users_identity.table_name, "users");
    assert_eq!(bronze_users_identity.qualified_name, "bronze.users");

    // Build dependency graph
    catalog.build_dependency_graph()?;

    // Test dependency graph functionality
    assert_eq!(catalog.dependency_graph.model_count(), 3);
    assert_eq!(catalog.dependency_graph.dependency_count(), 1); // silver.customers -> bronze.users

    // Test dependencies
    let silver_deps = catalog.get_dependencies("silver.customers");
    assert_eq!(silver_deps, vec!["bronze.users"]);

    let bronze_deps = catalog.get_dependencies("bronze.users");
    assert!(bronze_deps.is_empty()); // bronze.users has no internal dependencies

    // Test dependents
    let bronze_dependents = catalog.get_dependents("bronze.users");
    assert_eq!(bronze_dependents, vec!["silver.customers"]);

    let silver_dependents = catalog.get_dependents("silver.customers");
    assert!(silver_dependents.is_empty()); // nothing depends on silver.customers

    // Test execution order
    let execution_order = catalog.get_execution_order()?;
    assert!(execution_order.len() >= 2); // At least bronze.users and silver.customers
    
    // bronze.users should come before silver.customers in execution order
    let bronze_pos = execution_order.iter().position(|x| x == "bronze.users").unwrap();
    let silver_pos = execution_order.iter().position(|x| x == "silver.customers").unwrap();
    assert!(bronze_pos < silver_pos);

    // Test no circular dependencies
    assert!(!catalog.has_circular_dependencies());

    Ok(())
}

#[test]
fn test_discover_models_nonexistent_directory() -> Result<()> {
    // Try to discover models in a nonexistent directory
    let nonexistent_dir = Path::new("/path/that/does/not/exist");
    let mut catalog = ModelCatalog::new(nonexistent_dir.to_path_buf());

    // This should return an error
    let result = catalog.discover_models();
    assert!(result.is_err());

    Ok(())
}

// Helper function to create a test SQL file
fn create_test_sql_file(dir: &Path, filename: &str, content: &str) -> Result<()> {
    let file_path = dir.join(filename);
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}
