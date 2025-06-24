#[cfg(test)]
#[cfg(feature = "postgres")]
mod tests {
    use crate::execution::{create_engine_with_available_adapters, SqlDialect, ExecutionStatus};
    use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};
    use tokio;

    #[tokio::test]
    async fn test_postgres_connection_string_validation() {
        let engine = create_engine_with_available_adapters();
        
        // Test invalid connection string
        let invalid_connection = "mysql://user:pass@localhost:3306/db";
        let result = engine.execute_sql("SELECT 1", invalid_connection, SqlDialect::Postgres).await;
        
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Invalid PostgreSQL connection string"));
    }

    #[tokio::test]
    async fn test_postgres_execution_with_testcontainer() {
        // Start a PostgreSQL container using testcontainers-modules
        let postgres_container = postgres::Postgres::default()
            .start()
            .await
            .expect("Failed to start postgres container");

        let connection_string = format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            postgres_container.get_host().await.expect("Failed to get host"),
            postgres_container.get_host_port_ipv4(5432).await.expect("Failed to get host port")
        );

        // Create execution engine
        let engine = create_engine_with_available_adapters();
        
        // Test simple SQL execution
        let sql = "CREATE TABLE test_table (id SERIAL PRIMARY KEY, name VARCHAR(50))";
        let result = engine.execute_sql(sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(result.is_ok());
        let execution_result = result.unwrap();
        assert_eq!(execution_result.status, ExecutionStatus::Success);
        
        // Test INSERT
        let insert_sql = "INSERT INTO test_table (name) VALUES ('test_name')";
        let insert_result = engine.execute_sql(insert_sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(insert_result.is_ok());
        let insert_execution_result = insert_result.unwrap();
        assert_eq!(insert_execution_result.status, ExecutionStatus::Success);
        assert_eq!(insert_execution_result.rows_affected, 1);
    }

    #[tokio::test]
    async fn test_postgres_execution_with_invalid_sql() {
        // Start a PostgreSQL container using testcontainers-modules
        let postgres_container = postgres::Postgres::default()
            .start()
            .await
            .expect("Failed to start postgres container");

        let connection_string = format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            postgres_container.get_host().await.expect("Failed to get host"),
            postgres_container.get_host_port_ipv4(5432).await.expect("Failed to get host port")
        );

        // Create execution engine
        let engine = create_engine_with_available_adapters();
        
        // Test invalid SQL
        let invalid_sql = "INVALID SQL STATEMENT";
        let result = engine.execute_sql(invalid_sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(result.is_ok()); // The function returns Ok with Failed status
        let execution_result = result.unwrap();
        assert_eq!(execution_result.status, ExecutionStatus::Failed);
        assert!(execution_result.message.is_some());
        assert!(execution_result.message.unwrap().contains("SQL execution failed"));
    }

    #[tokio::test]
    async fn test_model_execution_end_to_end() {
        // Start a PostgreSQL container
        let postgres_container = postgres::Postgres::default()
            .start()
            .await
            .expect("Failed to start postgres container");

        let connection_string = format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            postgres_container.get_host().await.expect("Failed to get host"),
            postgres_container.get_host_port_ipv4(5432).await.expect("Failed to get host port")
        );

        // Create execution engine
        let engine = create_engine_with_available_adapters();
        
        // Test a complete model execution scenario
        // 1. Create a source table
        let create_source = "CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(100), email VARCHAR(100))";
        let result = engine.execute_sql(create_source, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        
        // 2. Insert some data
        let insert_data = "INSERT INTO users (name, email) VALUES ('John Doe', 'john@example.com'), ('Jane Smith', 'jane@example.com')";
        let result = engine.execute_sql(insert_data, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        let insert_result = result.unwrap();
        assert_eq!(insert_result.status, ExecutionStatus::Success);
        assert_eq!(insert_result.rows_affected, 2);
        
        // 3. Create a model (transformation)
        let create_model = "CREATE TABLE user_summary AS SELECT COUNT(*) as total_users, COUNT(DISTINCT email) as unique_emails FROM users";
        let result = engine.execute_sql(create_model, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        
        // 4. Verify the model worked
        let verify_model = "SELECT total_users, unique_emails FROM user_summary";
        let result = engine.execute_sql(verify_model, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
    }
}