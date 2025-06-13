use color_eyre::Result;
use color_eyre::eyre::{Context, eyre};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::dependency_graph::{DependencyGraph, ModelIdentity};
use crate::parser::{ModelMetadata, ModelParser};

/// Recursively find all SQL files in a directory
fn find_sql_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut sql_files = Vec::new();

    if dir.is_dir() {
        for entry_result in
            fs::read_dir(dir).wrap_err_with(|| format!("Failed to read directory: {:?}", dir))?
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
    /// Map of qualified model name to ModelMetadata
    pub models: HashMap<String, ModelMetadata>,
    /// Map of qualified model name to ModelIdentity
    pub model_identities: HashMap<String, ModelIdentity>,
    /// Dependency graph of models
    pub dependency_graph: DependencyGraph,
    /// Directory where models were discovered
    pub model_dir: PathBuf,
}

impl ModelCatalog {
    /// Create a new empty ModelCatalog
    pub fn new(model_dir: PathBuf) -> Self {
        Self {
            models: HashMap::new(),
            model_identities: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
            model_dir,
        }
    }

    /// Discover all SQL models in the specified directory
    pub fn discover_models(&mut self) -> Result<()> {
        // Check if the directory exists
        if !self.model_dir.exists() {
            return Err(eyre!(
                "Model directory does not exist: {:?}",
                self.model_dir
            ));
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
        // Create ModelIdentity from file path
        let model_identity = ModelIdentity::from_path(file_path.to_path_buf(), &self.model_dir)?;

        // Read the SQL file content
        let sql_content = fs::read_to_string(file_path)
            .wrap_err_with(|| format!("Failed to read SQL file: {:?}", file_path))?;

        // Create and parse the model using the qualified name
        let mut model = ModelMetadata::new(model_identity.qualified_name.clone());
        model.parse_model(&sql_content).map_err(|e| {
            eyre!(
                "Failed to parse model {}: {}",
                model_identity.qualified_name,
                e
            )
        })?;

        // Add the model and identity to the catalog
        self.models
            .insert(model_identity.qualified_name.clone(), model);
        self.model_identities
            .insert(model_identity.qualified_name.clone(), model_identity);

        Ok(())
    }

    /// Build a dependency graph from the discovered models
    pub fn build_dependency_graph(&mut self) -> Result<()> {
        // Clear the existing graph
        self.dependency_graph = DependencyGraph::new();

        // Add all models to the graph first
        for qualified_name in self.models.keys() {
            self.dependency_graph.add_model(qualified_name);
        }

        // Add dependencies based on model sources
        for (model_name, model) in &self.models {
            for source in &model.sources {
                // Check if the source is another model in our catalog
                if self.models.contains_key(&source.id) {
                    // Add dependency: model_name depends on source.id
                    self.dependency_graph
                        .add_dependency(model_name, &source.id)?;
                }
                // Note: We ignore external dependencies (sources not in our catalog)
                // as they represent external tables/views
            }
        }

        Ok(())
    }

    /// Get the execution order of all models
    pub fn get_execution_order(&self) -> Result<Vec<String>> {
        self.dependency_graph.execution_order()
    }

    /// Check if there are any circular dependencies
    pub fn has_circular_dependencies(&self) -> bool {
        self.dependency_graph.has_cycles()
    }

    /// Get all models that depend on the given model
    pub fn get_dependents(&self, model_name: &str) -> Vec<String> {
        self.dependency_graph.get_dependents(model_name)
    }

    /// Get all models that the given model depends on
    pub fn get_dependencies(&self, model_name: &str) -> Vec<String> {
        self.dependency_graph.get_dependencies(model_name)
    }
}
