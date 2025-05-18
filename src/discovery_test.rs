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

    // Create a few test SQL files
    create_test_sql_file(&model_dir, "model1.sql", "SELECT a, b FROM source1")?;
    create_test_sql_file(&model_dir, "model2.sql", "SELECT c, d FROM source2")?;
    
    // Create a subdirectory with more SQL files
    let subdir = model_dir.join("subdir");
    fs::create_dir(&subdir)?;
    create_test_sql_file(&subdir, "model3.sql", "SELECT e, f FROM source3")?;

    // Create a non-SQL file that should be ignored
    let non_sql_path = model_dir.join("not_a_model.txt");
    let mut non_sql_file = File::create(non_sql_path)?;
    writeln!(non_sql_file, "This is not a SQL file")?;

    // Discover models
    let mut catalog = ModelCatalog::new(model_dir);
    catalog.discover_models()?;

    // Verify that all SQL models were discovered
    assert_eq!(catalog.models.len(), 3);
    assert!(catalog.models.contains_key("model1"));
    assert!(catalog.models.contains_key("model2"));
    assert!(catalog.models.contains_key("model3"));
    
    // Verify that non-SQL files were ignored
    assert!(!catalog.models.contains_key("not_a_model"));

    // Verify model content
    let model1 = catalog.models.get("model1").unwrap();
    assert_eq!(model1.sources.len(), 1);
    assert_eq!(model1.sources.iter().next().unwrap(), "source_table");
    
    let model2 = catalog.models.get("model2").unwrap();
    assert_eq!(model2.sources.len(), 1);
    assert_eq!(model2.sources.iter().next().unwrap(), "source_table");
    
    let model3 = catalog.models.get("model3").unwrap();
    assert_eq!(model3.sources.len(), 1);
    assert_eq!(model3.sources.iter().next().unwrap(), "source_table");

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
