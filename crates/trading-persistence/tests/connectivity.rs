//! PostgreSQL integration test.
//!
//! This test is ignored by `cargo test` and runs only when requested. It needs
//! a reachable database and `DATABASE_URL`.

use trading_persistence::{Database, redact_connection_secrets};

#[tokio::test]
#[ignore = "requires PostgreSQL; see docs/development/README.md"]
async fn connects_migrates_and_reads_infrastructure_metadata() {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set to run the database integration test");
    let database = match Database::connect(&url) {
        Ok(database) => database,
        Err(error) => {
            let message = redact_connection_secrets(&error.to_string(), &url);
            panic!("DATABASE_URL was rejected: {message}");
        }
    };

    if let Err(error) = database.check_ready().await {
        let message = redact_connection_secrets(&error.to_string(), &url);
        panic!("postgres should accept a connection and apply migrations: {message}");
    }

    let purpose = match database.infrastructure_value("schema_purpose").await {
        Ok(purpose) => purpose,
        Err(error) => {
            let message = redact_connection_secrets(&error.to_string(), &url);
            panic!("infrastructure metadata should be readable: {message}");
        }
    };
    assert_eq!(purpose.as_deref(), Some("infrastructure-metadata-only"));
}
