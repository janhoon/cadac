use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use color_eyre::Result;
use color_eyre::eyre::{eyre, Context};

use crate::parser::{ModelMetadata, ModelParser};

/// Recursively find all SQL files in a directory
fn find_sql_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut sql_files = Vec::new();
    
    if dir.is_dir() {
        for entry_result in fs::read_dir(dir)
            .wrap_err_with(|| format!("Failed to read directory: {:?}", dir))?
        {
            let entry = entry_result
                .wrap_err_with(|| format!("Failed to read directory entry in {:?}", dir))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively process subdirectories
                let mut sub_files = find_sql_files(&path)?;
                sql_files.append(&mut sub_files);
            } else if let Some(extension) = path.extension() {
                // Add SQL files to the list
                if extension == "sql" {
                    sql_files.push(path);
                }
            }
        }
    }
    
    Ok(sql_files)
}

/// Represents a collection of models discovered from SQL files
pub struct ModelCatalog {
    /// Map of model name to ModelMetadata
    pub models: HashMap<String, ModelMetadata>,
    /// Directory where models were discovered
    pub model_dir: PathBuf,
}

impl ModelCatalog {
    /// Create a new empty ModelCatalog
    pub fn new(model_dir: PathBuf) -> Self {
        Self {
            models: HashMap::new(),
            model_dir,
        }
    }

    /// Discover all SQL models in the specified directory
    pub fn discover_models(&mut self) -> Result<()> {
        // Check if the directory exists
        if !self.model_dir.exists() {
            return Err(eyre!("Model directory does not exist: {:?}", self.model_dir));
        }

        // Find all SQL files in the directory
        let sql_files = find_sql_files(&self.model_dir)?;
        
        // Process each SQL file
        for file_path in sql_files {
            self.process_sql_file(&file_path)?;
        }

        Ok(())
    }

    /// Process a single SQL file and add it to the catalog
    fn process_sql_file(&mut self, file_path: &Path) -> Result<()> {
        // Extract model name from filename (without extension)
        let model_name = file_path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| eyre!("Failed to extract model name from path: {:?}", file_path))?
            .to_string();
        
        // Read the SQL file content
        let sql_content = fs::read_to_string(file_path)
            .wrap_err_with(|| format!("Failed to read SQL file: {:?}", file_path))?;
        
        // Create and parse the model
        let mut model = ModelMetadata::new(model_name.clone());
        model.parse_model(&sql_content)
            .map_err(|e| eyre!("Failed to parse model {}: {}", model_name, e))?;
        
        // Add the model to the catalog
        self.models.insert(model_name, model);
        
        Ok(())
    }

    /// Build a dependency graph from the discovered models
    pub fn build_dependency_graph(&self) -> Result<()> {
        // This will be implemented in the next step
        // For now, just return Ok
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_discover_models() -> Result<()> {
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

        // Discover models
        let mut catalog = ModelCatalog::new(model_dir);
        catalog.discover_models()?;

        // Verify that all models were discovered
        assert_eq!(catalog.models.len(), 3);
        assert!(catalog.models.contains_key("model1"));
        assert!(catalog.models.contains_key("model2"));
        assert!(catalog.models.contains_key("model3"));

        Ok(())
    }

    // Helper function to create a test SQL file
    fn create_test_sql_file(dir: &Path, filename: &str, content: &str) -> Result<()> {
        let file_path = dir.join(filename);
        let mut file = File::create(file_path)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
}
