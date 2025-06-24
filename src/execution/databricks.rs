// Databricks adapter implementation
// This will be implemented when databricks feature is added

use super::{DatabaseAdapter, DatabaseConnection, ExecutionResult, ExecutionStatus, SqlDialect};
use color_eyre::Result;

/// Databricks connection implementation (placeholder)
pub struct DatabricksConnection;

#[async_trait::async_trait]
impl DatabaseConnection for DatabricksConnection {
    async fn execute_sql(&self, _sql: &str) -> Result<ExecutionResult> {
        // TODO: Implement Databricks SQL execution
        unimplemented!("Databricks adapter not yet implemented")
    }

    fn dialect(&self) -> SqlDialect {
        SqlDialect::Databricks
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// Databricks adapter implementation (placeholder)
pub struct DatabricksAdapter;

#[async_trait::async_trait]
impl DatabaseAdapter for DatabricksAdapter {
    async fn connect(&self, _connection_string: &str) -> Result<Box<dyn DatabaseConnection>> {
        // TODO: Implement Databricks connection
        unimplemented!("Databricks adapter not yet implemented")
    }

    fn dialect(&self) -> SqlDialect {
        SqlDialect::Databricks
    }

    fn validate_connection_string(&self, _connection_string: &str) -> Result<()> {
        // TODO: Implement Databricks connection string validation
        unimplemented!("Databricks adapter not yet implemented")
    }
}