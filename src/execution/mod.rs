use color_eyre::Result;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Represents the result of executing a SQL statement
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub rows_affected: u64,
    pub execution_time: Duration,
    pub status: ExecutionStatus,
    pub message: Option<String>,
    pub started_at: SystemTime,
    pub query_hash: Option<String>,
}

impl ExecutionResult {
    pub fn new(status: ExecutionStatus) -> Self {
        Self {
            rows_affected: 0,
            execution_time: Duration::from_millis(0),
            status,
            message: None,
            started_at: SystemTime::now(),
            query_hash: None,
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_rows_affected(mut self, rows: u64) -> Self {
        self.rows_affected = rows;
        self
    }

    pub fn with_execution_time(mut self, duration: Duration) -> Self {
        self.execution_time = duration;
        self
    }

    pub fn with_query_hash(mut self, hash: String) -> Self {
        self.query_hash = Some(hash);
        self
    }
}

/// Status of SQL execution
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Success,
    Failed,
    Skipped,
}

/// SQL dialect types for different database platforms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SqlDialect {
    Postgres,
    Databricks,
    Snowflake,
}

/// Database connection trait for abstracting different database types
#[async_trait::async_trait]
pub trait DatabaseConnection: Send + Sync {
    async fn execute_sql(&self, sql: &str) -> Result<ExecutionResult>;
    fn dialect(&self) -> SqlDialect;
    async fn close(&self) -> Result<()>;
}

/// Database adapter trait for creating connections to different platforms
#[async_trait::async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn connect(&self, connection_string: &str) -> Result<Box<dyn DatabaseConnection>>;
    fn dialect(&self) -> SqlDialect;
    fn validate_connection_string(&self, connection_string: &str) -> Result<()>;
}

/// Options for running models
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub include_upstream: bool,
    pub include_downstream: bool,
    pub dry_run: bool,
    pub fail_fast: bool,
    pub target_database: Option<String>,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            include_upstream: false,
            include_downstream: false,
            dry_run: false,
            fail_fast: true,
            target_database: None,
        }
    }
}

/// Execution plan for running models
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub models: Vec<String>,
    pub execution_order: Vec<String>,
    pub dry_run: bool,
}

/// Main execution engine for orchestrating model runs
pub struct ExecutionEngine {
    adapters: HashMap<SqlDialect, Box<dyn DatabaseAdapter>>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        let adapters: HashMap<SqlDialect, Box<dyn DatabaseAdapter>> = HashMap::new();
        Self { adapters }
    }

    /// Register a database adapter for a specific dialect
    pub fn register_adapter(&mut self, dialect: SqlDialect, adapter: Box<dyn DatabaseAdapter>) {
        self.adapters.insert(dialect, adapter);
    }

    /// Get list of available database dialects
    pub fn available_dialects(&self) -> Vec<SqlDialect> {
        self.adapters.keys().cloned().collect()
    }

    /// Check if a dialect is supported
    pub fn supports_dialect(&self, dialect: &SqlDialect) -> bool {
        self.adapters.contains_key(dialect)
    }

    /// Execute SQL using the specified dialect
    pub async fn execute_sql(
        &self,
        sql: &str,
        connection_string: &str,
        dialect: SqlDialect,
    ) -> Result<ExecutionResult> {
        let adapter = self.adapters.get(&dialect)
            .ok_or_else(|| color_eyre::eyre::eyre!(
                "No adapter found for dialect: {:?}. Available dialects: {:?}", 
                dialect, 
                self.available_dialects()
            ))?;

        adapter.validate_connection_string(connection_string)?;
        let connection = adapter.connect(connection_string).await?;
        let result = connection.execute_sql(sql).await?;
        connection.close().await?;

        Ok(result)
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Optional database adapter modules
#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "databricks")]
pub mod databricks;

#[cfg(feature = "snowflake")]
pub mod snowflake;

/// Create an execution engine with all available adapters registered
pub fn create_engine_with_available_adapters() -> ExecutionEngine {
    let mut engine = ExecutionEngine::new();
    
    #[cfg(feature = "postgres")]
    {
        engine.register_adapter(SqlDialect::Postgres, Box::new(postgres::PostgresAdapter));
    }
    
    #[cfg(feature = "databricks")]
    {
        // engine.register_adapter(SqlDialect::Databricks, Box::new(databricks::DatabricksAdapter));
    }
    
    #[cfg(feature = "snowflake")]
    {
        // engine.register_adapter(SqlDialect::Snowflake, Box::new(snowflake::SnowflakeAdapter));
    }
    
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_options_default() {
        let options = RunOptions::default();
        assert!(!options.include_upstream);
        assert!(!options.include_downstream);
        assert!(!options.dry_run);
        assert!(options.fail_fast);
        assert!(options.target_database.is_none());
    }

    #[test]
    fn test_execution_engine_creation() {
        let engine = ExecutionEngine::new();
        assert_eq!(engine.available_dialects().len(), 0);
    }

    #[test]
    fn test_create_engine_with_available_adapters() {
        let engine = create_engine_with_available_adapters();
        
        #[cfg(feature = "postgres")]
        {
            assert!(engine.supports_dialect(&SqlDialect::Postgres));
        }
        
        #[cfg(not(feature = "postgres"))]
        {
            assert!(!engine.supports_dialect(&SqlDialect::Postgres));
        }
    }
}
