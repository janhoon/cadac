use super::{DatabaseAdapter, DatabaseConnection, ExecutionResult, ExecutionStatus, SqlDialect};
use color_eyre::Result;
use tokio_postgres::{Client, NoTls, Transaction};

/// PostgreSQL connection implementation
pub struct PostgresConnection {
    client: Client,
}

#[async_trait::async_trait]
impl DatabaseConnection for PostgresConnection {
    async fn execute_sql(&self, sql: &str) -> Result<ExecutionResult> {
        let start_time = std::time::Instant::now();
        let query_hash = format!("{:x}", md5::compute(sql.as_bytes()));
        
        match self.client.execute(sql, &[]).await {
            Ok(rows_affected) => {
                let execution_time = start_time.elapsed();
                Ok(ExecutionResult::new(ExecutionStatus::Success)
                    .with_rows_affected(rows_affected)
                    .with_execution_time(execution_time)
                    .with_query_hash(query_hash)
                    .with_message(format!("Successfully executed SQL, {} rows affected", rows_affected)))
            }
            Err(e) => {
                let execution_time = start_time.elapsed();
                let error_details = self.categorize_error(&e);
                Ok(ExecutionResult::new(ExecutionStatus::Failed)
                    .with_execution_time(execution_time)
                    .with_query_hash(query_hash)
                    .with_message(format!("SQL execution failed [{}]: {}", error_details.category, error_details.message)))
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

/// Error category for better error handling
#[derive(Debug)]
pub struct ErrorDetails {
    pub category: String,
    pub message: String,
    pub is_recoverable: bool,
}

impl PostgresConnection {
    /// Categorize PostgreSQL errors for better error handling
    fn categorize_error(&self, error: &tokio_postgres::Error) -> ErrorDetails {
        let error_string = error.to_string();
        
        // Check for specific PostgreSQL error codes and patterns
        if error_string.contains("syntax error") || error_string.contains("42601") {
            ErrorDetails {
                category: "SYNTAX_ERROR".to_string(),
                message: "SQL syntax error detected".to_string(),
                is_recoverable: false,
            }
        } else if error_string.contains("relation") && error_string.contains("does not exist") {
            ErrorDetails {
                category: "MISSING_RELATION".to_string(),
                message: "Referenced table or view does not exist".to_string(),
                is_recoverable: false,
            }
        } else if error_string.contains("column") && error_string.contains("does not exist") {
            ErrorDetails {
                category: "MISSING_COLUMN".to_string(),
                message: "Referenced column does not exist".to_string(),
                is_recoverable: false,
            }
        } else if error_string.contains("permission denied") || error_string.contains("42501") {
            ErrorDetails {
                category: "PERMISSION_DENIED".to_string(),
                message: "Insufficient permissions to execute query".to_string(),
                is_recoverable: false,
            }
        } else if error_string.contains("duplicate key") || error_string.contains("23505") {
            ErrorDetails {
                category: "DUPLICATE_KEY".to_string(),
                message: "Unique constraint violation".to_string(),
                is_recoverable: false,
            }
        } else if error_string.contains("connection") {
            ErrorDetails {
                category: "CONNECTION_ERROR".to_string(),
                message: "Database connection issue".to_string(),
                is_recoverable: true,
            }
        } else if error_string.contains("timeout") {
            ErrorDetails {
                category: "TIMEOUT".to_string(),
                message: "Query execution timeout".to_string(),
                is_recoverable: true,
            }
        } else {
            ErrorDetails {
                category: "UNKNOWN_ERROR".to_string(),
                message: format!("Unrecognized error: {}", error),
                is_recoverable: false,
            }
        }
    }

    /// Execute multiple SQL statements within a transaction
    pub async fn execute_transaction(&mut self, sql_statements: Vec<&str>) -> Result<Vec<ExecutionResult>> {
        let transaction = self.client.transaction().await?;
        let mut results = Vec::new();
        let total_start = std::time::Instant::now();

        for sql in sql_statements.iter() {
            let start_time = std::time::Instant::now();
            let query_hash = format!("{:x}", md5::compute(sql.as_bytes()));
            
            match transaction.execute(*sql, &[]).await {
                Ok(rows_affected) => {
                    let execution_time = start_time.elapsed();
                    results.push(ExecutionResult::new(ExecutionStatus::Success)
                        .with_rows_affected(rows_affected)
                        .with_execution_time(execution_time)
                        .with_query_hash(query_hash)
                        .with_message(format!("Successfully executed SQL in transaction, {} rows affected", rows_affected)));
                }
                Err(e) => {
                    let execution_time = start_time.elapsed();
                    // Create error details inline to avoid borrowing issues
                    let error_string = e.to_string();
                    let (category, message) = if error_string.contains("syntax error") || error_string.contains("42601") {
                        ("SYNTAX_ERROR", "SQL syntax error detected")
                    } else if error_string.contains("relation") && error_string.contains("does not exist") {
                        ("MISSING_RELATION", "Referenced table or view does not exist")
                    } else if error_string.contains("column") && error_string.contains("does not exist") {
                        ("MISSING_COLUMN", "Referenced column does not exist")
                    } else if error_string.contains("permission denied") || error_string.contains("42501") {
                        ("PERMISSION_DENIED", "Insufficient permissions to execute query")
                    } else if error_string.contains("duplicate key") || error_string.contains("23505") {
                        ("DUPLICATE_KEY", "Unique constraint violation")
                    } else if error_string.contains("connection") {
                        ("CONNECTION_ERROR", "Database connection issue")
                    } else if error_string.contains("timeout") {
                        ("TIMEOUT", "Query execution timeout")
                    } else {
                        ("UNKNOWN_ERROR", "Unrecognized error")
                    };
                    
                    let failed_result = ExecutionResult::new(ExecutionStatus::Failed)
                        .with_execution_time(execution_time)
                        .with_query_hash(query_hash)
                        .with_message(format!("SQL execution failed in transaction [{}]: {}", category, message));
                    results.push(failed_result);
                    
                    // Rollback transaction on failure
                    if let Err(rollback_err) = transaction.rollback().await {
                        return Err(color_eyre::eyre::eyre!(
                            "Transaction failed and rollback also failed. Original error: {}, Rollback error: {}", 
                            e, rollback_err
                        ));
                    }
                    
                    return Ok(results);
                }
            }
        }

        // Commit transaction if all statements succeeded
        if let Err(e) = transaction.commit().await {
            return Err(color_eyre::eyre::eyre!("Failed to commit transaction: {}", e));
        }

        let total_time = total_start.elapsed();
        
        // Add a summary result for the transaction
        results.push(ExecutionResult::new(ExecutionStatus::Success)
            .with_rows_affected(results.iter().map(|r| r.rows_affected).sum())
            .with_execution_time(total_time)
            .with_message(format!("Transaction completed successfully. {} statements executed.", sql_statements.len())));

        Ok(results)
    }

    /// Test the connection by executing a simple query
    pub async fn test_connection(&self) -> Result<bool> {
        match self.client.execute("SELECT 1", &[]).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get PostgreSQL version information
    pub async fn get_version(&self) -> Result<String> {
        let row = self.client.query_one("SELECT version()", &[]).await?;
        let version: String = row.get(0);
        Ok(version)
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
