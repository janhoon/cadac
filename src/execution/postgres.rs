use super::{DatabaseAdapter, DatabaseConnection, ExecutionResult, ExecutionStatus, SqlDialect};
use color_eyre::Result;
use tokio_postgres::{Client, NoTls};

/// PostgreSQL connection implementation
pub struct PostgresConnection {
    client: Client,
}

#[async_trait::async_trait]
impl DatabaseConnection for PostgresConnection {
    async fn execute_sql(&self, sql: &str) -> Result<ExecutionResult> {
        let start_time = std::time::Instant::now();
        
        match self.client.execute(sql, &[]).await {
            Ok(rows_affected) => {
                let execution_time = start_time.elapsed();
                Ok(ExecutionResult {
                    rows_affected,
                    execution_time,
                    status: ExecutionStatus::Success,
                    message: Some(format!("Successfully executed SQL, {} rows affected", rows_affected)),
                })
            }
            Err(e) => {
                let execution_time = start_time.elapsed();
                Ok(ExecutionResult {
                    rows_affected: 0,
                    execution_time,
                    status: ExecutionStatus::Failed,
                    message: Some(format!("SQL execution failed: {}", e)),
                })
            }
        }
    }

    fn dialect(&self) -> SqlDialect {
        SqlDialect::Postgres
    }

    async fn close(&self) -> Result<()> {
        // PostgreSQL client doesn't need explicit closing in tokio-postgres
        Ok(())
    }
}

/// PostgreSQL adapter implementation
pub struct PostgresAdapter;

#[async_trait::async_trait]
impl DatabaseAdapter for PostgresAdapter {
    async fn connect(&self, connection_string: &str) -> Result<Box<dyn DatabaseConnection>> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        
        // Spawn the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        Ok(Box::new(PostgresConnection { client }))
    }

    fn dialect(&self) -> SqlDialect {
        SqlDialect::Postgres
    }

    fn validate_connection_string(&self, connection_string: &str) -> Result<()> {
        // Basic validation for PostgreSQL connection string
        if !connection_string.starts_with("postgresql://") && !connection_string.starts_with("postgres://") {
            return Err(color_eyre::eyre::eyre!(
                "Invalid PostgreSQL connection string. Must start with 'postgresql://' or 'postgres://'"
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgres_adapter_validation() {
        let adapter = PostgresAdapter;
        
        // Valid connection strings
        assert!(adapter.validate_connection_string("postgresql://user:pass@localhost:5432/db").is_ok());
        assert!(adapter.validate_connection_string("postgres://user:pass@localhost:5432/db").is_ok());
        
        // Invalid connection strings
        assert!(adapter.validate_connection_string("mysql://user:pass@localhost:3306/db").is_err());
        assert!(adapter.validate_connection_string("invalid_string").is_err());
    }

    #[test]
    fn test_postgres_adapter_dialect() {
        let adapter = PostgresAdapter;
        assert_eq!(adapter.dialect(), SqlDialect::Postgres);
    }
}