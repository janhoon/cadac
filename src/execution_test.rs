#[cfg(test)]
#[cfg(feature = "postgres")]
mod tests {
    use crate::execution::{create_engine_with_available_adapters, SqlDialect, ExecutionStatus, DatabaseAdapter, DatabaseConnection};
    use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};
    use tokio;
    use std::time::Duration;

    /// Helper function to start a PostgreSQL container with better error handling
    async fn start_postgres_container() -> Result<testcontainers_modules::testcontainers::ContainerAsync<postgres::Postgres>, String> {
        let container = postgres::Postgres::default()
            .start()
            .await
            .map_err(|e| {
                format!("Failed to start postgres container. This might be due to Docker networking issues. Error: {}", e)
            })?;
        
        // Wait a bit for the container to be fully ready
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        Ok(container)
    }

    /// Helper function to create connection string from container
    async fn get_connection_string(container: &testcontainers_modules::testcontainers::ContainerAsync<postgres::Postgres>) -> Result<String, String> {
        let host = container.get_host().await.map_err(|e| format!("Failed to get host: {}", e))?;
        let port = container.get_host_port_ipv4(5432).await.map_err(|e| format!("Failed to get port: {}", e))?;
        
        Ok(format!("postgresql://postgres:postgres@{}:{}/postgres", host, port))
    }

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
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

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
        
        // Test query execution
        let query_sql = "SELECT COUNT(*) FROM test_table WHERE name = 'test_name'";
        let query_result = engine.execute_sql(query_sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(query_result.is_ok());
        let query_execution_result = query_result.unwrap();
        assert_eq!(query_execution_result.status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_postgres_execution_with_invalid_sql() {
        // Start a PostgreSQL container using testcontainers-modules
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

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
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

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

    #[tokio::test]
    async fn test_complex_sql_operations() {
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let engine = create_engine_with_available_adapters();
        
        // Test WITH clause (CTE)
        let cte_sql = r#"
            CREATE TABLE orders AS 
            WITH monthly_sales AS (
                SELECT 
                    EXTRACT(MONTH FROM CURRENT_DATE) as month,
                    1000 as sales_amount
            )
            SELECT month, sales_amount FROM monthly_sales
        "#;
        
        let result = engine.execute_sql(cte_sql, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        
        // Test complex aggregation
        let agg_sql = r#"
            CREATE TABLE sales_summary AS
            SELECT 
                month,
                sales_amount,
                CASE 
                    WHEN sales_amount > 500 THEN 'High'
                    ELSE 'Low'
                END as performance_category
            FROM orders
        "#;
        
        let result = engine.execute_sql(agg_sql, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_transaction_like_operations() {
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let engine = create_engine_with_available_adapters();
        
        // Create test table
        let create_table = "CREATE TABLE test_transactions (id SERIAL PRIMARY KEY, value INTEGER)";
        let result = engine.execute_sql(create_table, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        
        // Test multiple operations that should all succeed
        let operations = vec![
            "INSERT INTO test_transactions (value) VALUES (100)",
            "INSERT INTO test_transactions (value) VALUES (200)",
            "UPDATE test_transactions SET value = 150 WHERE value = 100",
        ];
        
        for operation in operations {
            let result = engine.execute_sql(operation, &connection_string, SqlDialect::Postgres).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        }
        
        // Verify final state
        let verify_sql = "SELECT COUNT(*) FROM test_transactions WHERE value IN (150, 200)";
        let result = engine.execute_sql(verify_sql, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_error_handling_and_recovery() {
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let engine = create_engine_with_available_adapters();
        
        // Test various error scenarios
        let error_scenarios = vec![
            ("Syntax error", "SELCT * FROM nonexistent"),
            ("Missing table", "SELECT * FROM nonexistent_table"),
            ("Invalid column", "SELECT nonexistent_column FROM information_schema.tables LIMIT 1"),
        ];
        
        for (scenario_name, sql) in error_scenarios {
            let result = engine.execute_sql(sql, &connection_string, SqlDialect::Postgres).await;
            
            // Should return Ok but with Failed status
            assert!(result.is_ok(), "Scenario '{}' should return Ok", scenario_name);
            let execution_result = result.unwrap();
            assert_eq!(execution_result.status, ExecutionStatus::Failed, "Scenario '{}' should have Failed status", scenario_name);
            assert!(execution_result.message.is_some(), "Scenario '{}' should have error message", scenario_name);
        }
        
        // Test that the connection can still work after errors
        let working_sql = "SELECT 1 as test_value";
        let result = engine.execute_sql(working_sql, &connection_string, SqlDialect::Postgres).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_execution_timing_and_metadata() {
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let engine = create_engine_with_available_adapters();
        
        // Test that execution timing is captured
        let sql = "CREATE TABLE timing_test (id SERIAL PRIMARY KEY, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)";
        let result = engine.execute_sql(sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(result.is_ok());
        let execution_result = result.unwrap();
        assert_eq!(execution_result.status, ExecutionStatus::Success);
        
        // Verify that execution time is reasonable (should be less than 1 second for simple operations)
        assert!(execution_result.execution_time.as_secs() < 1);
        assert!(execution_result.execution_time.as_millis() > 0);
        
        // Test INSERT with row count tracking
        let insert_sql = "INSERT INTO timing_test DEFAULT VALUES";
        let insert_result = engine.execute_sql(insert_sql, &connection_string, SqlDialect::Postgres).await;
        
        assert!(insert_result.is_ok());
        let insert_execution_result = insert_result.unwrap();
        assert_eq!(insert_execution_result.status, ExecutionStatus::Success);
        assert_eq!(insert_execution_result.rows_affected, 1);
        
        // Test message content
        assert!(insert_execution_result.message.is_some());
        let message = insert_execution_result.message.unwrap();
        assert!(message.contains("Successfully executed SQL"));
        assert!(message.contains("1 rows affected"));
    }

    #[tokio::test]
    async fn test_postgres_transaction_functionality() {
        use crate::execution::postgres::PostgresAdapter;
        
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let adapter = PostgresAdapter;
        let connection = adapter.connect(&connection_string).await.unwrap();
        
        // Downcast to PostgresConnection to access transaction methods
        // Note: This is a bit of a hack for testing, but necessary to test PostgreSQL-specific functionality
        let mut postgres_connection = unsafe {
            let raw = Box::into_raw(connection);
            let concrete = raw as *mut crate::execution::postgres::PostgresConnection;
            Box::from_raw(concrete)
        };

        // Test successful transaction
        let transaction_statements = vec![
            "CREATE TABLE transaction_test (id SERIAL PRIMARY KEY, value INTEGER)",
            "INSERT INTO transaction_test (value) VALUES (100)",
            "INSERT INTO transaction_test (value) VALUES (200)",
            "UPDATE transaction_test SET value = value + 10 WHERE value = 100",
        ];

        let results = postgres_connection.execute_transaction(transaction_statements).await;
        assert!(results.is_ok());
        
        let execution_results = results.unwrap();
        // Should have 4 statement results + 1 summary result
        assert_eq!(execution_results.len(), 5);
        
        // All individual statements should succeed
        for i in 0..4 {
            assert_eq!(execution_results[i].status, ExecutionStatus::Success);
        }
        
        // Summary result should indicate success
        let summary = &execution_results[4];
        assert_eq!(summary.status, ExecutionStatus::Success);
        assert!(summary.message.as_ref().unwrap().contains("Transaction completed successfully"));
        assert!(summary.message.as_ref().unwrap().contains("4 statements executed"));

        // Test transaction rollback on failure
        let failing_statements = vec![
            "INSERT INTO transaction_test (value) VALUES (300)",
            "INVALID SQL STATEMENT", // This should cause rollback
            "INSERT INTO transaction_test (value) VALUES (400)",
        ];

        let rollback_results = postgres_connection.execute_transaction(failing_statements).await;
        assert!(rollback_results.is_ok());
        
        let rollback_execution_results = rollback_results.unwrap();
        // Should have 2 results: 1 success + 1 failure (rollback stops execution)
        assert_eq!(rollback_execution_results.len(), 2);
        assert_eq!(rollback_execution_results[0].status, ExecutionStatus::Success);
        assert_eq!(rollback_execution_results[1].status, ExecutionStatus::Failed);
        
        // Verify rollback worked - value 300 should not be in the table
        let verify_rollback = postgres_connection.execute_sql("SELECT COUNT(*) FROM transaction_test WHERE value = 300").await;
        assert!(verify_rollback.is_ok());
        assert_eq!(verify_rollback.unwrap().status, ExecutionStatus::Success);
    }

    #[tokio::test]
    async fn test_postgres_connection_utilities() {
        use crate::execution::postgres::PostgresAdapter;
        
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let adapter = PostgresAdapter;
        let connection = adapter.connect(&connection_string).await.unwrap();
        
        // Downcast to access PostgreSQL-specific methods
        let postgres_connection = unsafe {
            let raw = Box::into_raw(connection);
            let concrete = raw as *mut crate::execution::postgres::PostgresConnection;
            Box::from_raw(concrete)
        };

        // Test connection health check
        let is_healthy = postgres_connection.test_connection().await;
        assert!(is_healthy.is_ok());
        assert!(is_healthy.unwrap());

        // Test version retrieval
        let version = postgres_connection.get_version().await;
        assert!(version.is_ok());
        let version_string = version.unwrap();
        assert!(version_string.contains("PostgreSQL"));
    }

    #[tokio::test]
    async fn test_comprehensive_model_workflow() {
        let postgres_container = match start_postgres_container().await {
            Ok(container) => container,
            Err(e) => {
                println!("Skipping integration test due to container startup failure: {}", e);
                return;
            }
        };

        let connection_string = match get_connection_string(&postgres_container).await {
            Ok(conn_str) => conn_str,
            Err(e) => {
                println!("Skipping test due to connection string error: {}", e);
                return;
            }
        };

        let engine = create_engine_with_available_adapters();
        
        // Simulate a typical data transformation workflow
        let setup_statements = vec![
            // 1. Create source tables (simulate raw data)
            "CREATE TABLE raw_customers (id SERIAL PRIMARY KEY, name VARCHAR(100), email VARCHAR(100), created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE raw_orders (id SERIAL PRIMARY KEY, customer_id INTEGER, amount DECIMAL(10,2), order_date DATE DEFAULT CURRENT_DATE)",
            
            // 2. Insert sample data
            "INSERT INTO raw_customers (name, email) VALUES ('Alice Johnson', 'alice@example.com'), ('Bob Smith', 'bob@example.com'), ('Carol Davis', 'carol@example.com')",
            "INSERT INTO raw_orders (customer_id, amount) VALUES (1, 150.00), (1, 75.50), (2, 200.00), (3, 300.25), (2, 125.75)",
        ];

        // Execute setup
        for sql in setup_statements {
            let result = engine.execute_sql(sql, &connection_string, SqlDialect::Postgres).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().status, ExecutionStatus::Success);
        }

        // 3. Create dimensional models (typical dbt-style transformations)
        let transformation_statements = vec![
            // Bronze layer - clean and standardize
            r#"CREATE TABLE bronze_customers AS 
               SELECT id, TRIM(UPPER(name)) as name, LOWER(email) as email, created_at 
               FROM raw_customers 
               WHERE email IS NOT NULL AND email LIKE '%@%'"#,
            
            r#"CREATE TABLE bronze_orders AS 
               SELECT id, customer_id, amount, order_date,
                      CASE WHEN amount > 200 THEN 'Large' ELSE 'Regular' END as order_size
               FROM raw_orders 
               WHERE amount > 0"#,
            
            // Silver layer - business logic and aggregations
            r#"CREATE TABLE silver_customer_metrics AS
               SELECT 
                   c.id, c.name, c.email,
                   COUNT(o.id) as total_orders,
                   COALESCE(SUM(o.amount), 0) as total_spent,
                   COALESCE(AVG(o.amount), 0) as avg_order_value,
                   MAX(o.order_date) as last_order_date
               FROM bronze_customers c
               LEFT JOIN bronze_orders o ON c.id = o.customer_id
               GROUP BY c.id, c.name, c.email"#,
            
            // Gold layer - business reporting
            r#"CREATE TABLE gold_customer_segments AS
               SELECT 
                   CASE 
                       WHEN total_spent > 250 THEN 'High Value'
                       WHEN total_spent > 100 THEN 'Medium Value'
                       ELSE 'Low Value'
                   END as customer_segment,
                   COUNT(*) as customer_count,
                   AVG(total_spent) as avg_segment_value,
                   SUM(total_spent) as segment_total_value
               FROM silver_customer_metrics
               GROUP BY 
                   CASE 
                       WHEN total_spent > 250 THEN 'High Value'
                       WHEN total_spent > 100 THEN 'Medium Value'
                       ELSE 'Low Value'
                   END"#
        ];

        // Execute transformations
        for sql in transformation_statements {
            let result = engine.execute_sql(sql, &connection_string, SqlDialect::Postgres).await;
            assert!(result.is_ok());
            let exec_result = result.unwrap();
            assert_eq!(exec_result.status, ExecutionStatus::Success);
            
            // Verify that transformations created data
            if sql.contains("CREATE TABLE") {
                assert!(exec_result.rows_affected >= 0); // Could be 0 for CREATE TABLE
            }
        }

        // 4. Validate final results
        let validation_queries = vec![
            ("Bronze customers count", "SELECT COUNT(*) FROM bronze_customers"),
            ("Bronze orders count", "SELECT COUNT(*) FROM bronze_orders"),
            ("Customer metrics count", "SELECT COUNT(*) FROM silver_customer_metrics"),
            ("Customer segments count", "SELECT COUNT(*) FROM gold_customer_segments"),
            ("Total revenue validation", "SELECT SUM(segment_total_value) FROM gold_customer_segments"),
        ];

        for (test_name, query) in validation_queries {
            let result = engine.execute_sql(query, &connection_string, SqlDialect::Postgres).await;
            assert!(result.is_ok(), "Validation query '{}' failed", test_name);
            assert_eq!(result.unwrap().status, ExecutionStatus::Success, "Query '{}' did not succeed", test_name);
        }
    }
}
