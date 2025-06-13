use color_eyre::Result;
use color_eyre::eyre::{Context, eyre};
use petgraph::algo::{is_cyclic_directed, toposort};
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents the identity of a model based on its file path and schema organization
#[derive(Debug, Clone, PartialEq)]
pub struct ModelIdentity {
    pub file_path: PathBuf,
    pub table_name: String,
    pub schema_name: String,
    pub qualified_name: String,
}

impl ModelIdentity {
    /// Create a ModelIdentity from a file path relative to the models root directory
    /// Schema is the first directory after models root, regardless of nesting
    /// Examples:
    /// - models/test/users/user_model.sql -> schema: "test", table: "user_model"
    /// - models/test/test_model.sql -> schema: "test", table: "test_model"
    pub fn from_path(file_path: PathBuf, models_root: &Path) -> Result<Self> {
        // Get the relative path from models root
        let relative_path = file_path.strip_prefix(models_root).wrap_err_with(|| {
            format!(
                "File path {:?} is not within models root {:?}",
                file_path, models_root
            )
        })?;

        // Extract schema name from the first directory component after models root
        let schema_name = relative_path
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_str())
            .ok_or_else(|| eyre!("Cannot extract schema from path: {:?}", relative_path))?
            .to_string();

        // Extract table name from filename (without .sql extension)
        let table_name = file_path
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre!("Cannot extract table name from path: {:?}", file_path))?
            .to_string();

        // Create qualified name
        let qualified_name = format!("{}.{}", schema_name, table_name);

        Ok(ModelIdentity {
            file_path,
            table_name,
            schema_name,
            qualified_name,
        })
    }
}

/// Represents a dependency graph of models using petgraph
#[derive(Debug)]
pub struct DependencyGraph {
    /// The underlying directed graph where nodes are model qualified names
    /// and edges represent dependencies (A -> B means A depends on B)
    graph: Graph<String, ()>,
    /// Map from qualified model name to node index for efficient lookups
    node_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_indices: HashMap::new(),
        }
    }

    /// Add a model to the graph if it doesn't already exist
    /// Returns the NodeIndex for the model
    pub fn add_model(&mut self, qualified_name: &str) -> NodeIndex {
        if let Some(&node_idx) = self.node_indices.get(qualified_name) {
            return node_idx;
        }

        let node_idx = self.graph.add_node(qualified_name.to_string());
        self.node_indices
            .insert(qualified_name.to_string(), node_idx);
        node_idx
    }

    /// Add a dependency relationship between two models
    /// from_model depends on to_model (from_model -> to_model)
    pub fn add_dependency(&mut self, from_model: &str, to_model: &str) -> Result<()> {
        let from_idx = self.add_model(from_model);
        let to_idx = self.add_model(to_model);

        // Add edge: from_model -> to_model (from depends on to)
        self.graph.add_edge(from_idx, to_idx, ());

        Ok(())
    }

    /// Check if the dependency graph has any cycles
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }

    /// Get the execution order of models using topological sorting
    /// Returns models in the order they should be executed (dependencies first)
    pub fn execution_order(&self) -> Result<Vec<String>> {
        match toposort(&self.graph, None) {
            Ok(node_indices) => {
                let mut execution_order: Vec<String> = node_indices
                    .iter()
                    .map(|&idx| self.graph[idx].clone())
                    .collect();
                // Reverse to get proper execution order (dependencies first)
                execution_order.reverse();
                Ok(execution_order)
            }
            Err(_) => Err(eyre!(
                "Cannot determine execution order: circular dependency detected"
            )),
        }
    }

    /// Get all models that depend on the given model (impact analysis)
    pub fn get_dependents(&self, model: &str) -> Vec<String> {
        if let Some(&node_idx) = self.node_indices.get(model) {
            self.graph
                .neighbors_directed(node_idx, Direction::Incoming)
                .map(|idx| self.graph[idx].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all models that the given model depends on (lineage tracking)
    pub fn get_dependencies(&self, model: &str) -> Vec<String> {
        if let Some(&node_idx) = self.node_indices.get(model) {
            self.graph
                .neighbors_directed(node_idx, Direction::Outgoing)
                .map(|idx| self.graph[idx].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    // /// Get all models in the graph
    // pub fn get_all_models(&self) -> Vec<String> {
    //     self.graph.node_weights().cloned().collect()
    // }

    /// Get the number of models in the graph
    pub fn model_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of dependencies in the graph
    pub fn dependency_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_model_identity_from_path() -> Result<()> {
        let models_root = PathBuf::from("models");
        let file_path = PathBuf::from("models/bronze/users.sql");

        let identity = ModelIdentity::from_path(file_path.clone(), &models_root)?;

        assert_eq!(identity.file_path, file_path);
        assert_eq!(identity.table_name, "users");
        assert_eq!(identity.schema_name, "bronze");
        assert_eq!(identity.qualified_name, "bronze.users");

        Ok(())
    }

    #[test]
    fn test_model_identity_nested_path() -> Result<()> {
        let models_root = PathBuf::from("models");
        let file_path = PathBuf::from("models/test/users/user_model.sql");
        let identity = ModelIdentity::from_path(file_path.clone(), &models_root)?;

        assert_eq!(identity.table_name, "user_model");
        assert_eq!(identity.schema_name, "test");
        assert_eq!(identity.qualified_name, "test.user_model");

        let deeper_file_path = PathBuf::from("models/test/users/deeper/user_model.sql");
        let deeper_identity = ModelIdentity::from_path(deeper_file_path.clone(), &models_root)?;

        assert_eq!(deeper_identity.table_name, "user_model");
        assert_eq!(deeper_identity.schema_name, "test");
        assert_eq!(deeper_identity.qualified_name, "test.user_model");

        Ok(())
    }

    #[test]
    fn test_dependency_graph_basic() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // Add models
        graph.add_model("bronze.users");
        graph.add_model("gold.orders");

        // Add dependency: gold.orders depends on bronze.users
        graph.add_dependency("gold.orders", "bronze.users")?;

        assert_eq!(graph.model_count(), 2);
        assert_eq!(graph.dependency_count(), 1);
        assert!(!graph.has_cycles());

        Ok(())
    }

    #[test]
    fn test_execution_order() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // Create a dependency chain: A -> B -> C
        graph.add_dependency("B", "A")?;
        graph.add_dependency("C", "B")?;

        let execution_order = graph.execution_order()?;

        // A should come first, then B, then C
        assert_eq!(execution_order, vec!["A", "B", "C"]);

        Ok(())
    }

    #[test]
    fn test_cycle_detection() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // Create a cycle: A -> B -> A
        graph.add_dependency("A", "B")?;
        graph.add_dependency("B", "A")?;

        assert!(graph.has_cycles());
        assert!(graph.execution_order().is_err());

        Ok(())
    }

    #[test]
    fn test_impact_analysis() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // bronze.users <- gold.orders
        // bronze.users <- silver.customers
        graph.add_dependency("gold.orders", "bronze.users")?;
        graph.add_dependency("silver.customers", "bronze.users")?;

        let dependents = graph.get_dependents("bronze.users");
        assert_eq!(dependents.len(), 2);
        assert!(dependents.contains(&"gold.orders".to_string()));
        assert!(dependents.contains(&"silver.customers".to_string()));

        Ok(())
    }

    #[test]
    fn test_lineage_tracking() -> Result<()> {
        let mut graph = DependencyGraph::new();

        // gold.orders -> bronze.users -> sources.users
        graph.add_dependency("gold.orders", "bronze.users")?;
        graph.add_dependency("bronze.users", "sources.users")?;

        let dependencies = graph.get_dependencies("gold.orders");
        assert_eq!(dependencies, vec!["bronze.users"]);

        let dependencies = graph.get_dependencies("bronze.users");
        assert_eq!(dependencies, vec!["sources.users"]);

        Ok(())
    }
}
