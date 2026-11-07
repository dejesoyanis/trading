use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

use crate::endpoint::database_endpoint;
use crate::error::{DatabaseUrlProblem, PersistenceError};

/// Lazy PostgreSQL pool and infrastructure-migration state.
///
/// Connecting does not open a socket. The first successful readiness check
/// applies the infrastructure migration and then runs `SELECT 1`.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
    migrated: Arc<AtomicBool>,
    migrate_lock: Arc<Mutex<()>>,
}

impl Database {
    /// Builds a lazy pool from a PostgreSQL URL.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::InvalidUrl`] when the URL is rejected.
    /// The error does not include the URL.
    pub fn connect(database_url: &str) -> Result<Self, PersistenceError> {
        let _endpoint = database_endpoint(database_url)?;
        let options = PgConnectOptions::from_str(database_url)
            .map_err(|_| PersistenceError::InvalidUrl(DatabaseUrlProblem::Rejected))?
            .application_name("trading-api");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect_lazy_with(options);
        Ok(Self {
            pool,
            migrated: Arc::new(AtomicBool::new(false)),
            migrate_lock: Arc::new(Mutex::new(())),
        })
    }

    /// Applies the infrastructure migration when needed, then checks connectivity.
    ///
    /// # Errors
    ///
    /// Returns a migration or query error when PostgreSQL cannot be reached or
    /// the infrastructure migration cannot be applied. Callers that log the
    /// error must pass it through [`crate::redact_connection_secrets`].
    pub async fn check_ready(&self) -> Result<(), PersistenceError> {
        if !self.migrated.load(Ordering::Acquire) {
            let _guard = self.migrate_lock.lock().await;
            if !self.migrated.load(Ordering::Acquire) {
                sqlx::migrate!("./migrations")
                    .run(&self.pool)
                    .await
                    .map_err(PersistenceError::Migrate)?;
                self.migrated.store(true, Ordering::Release);
            }
        }
        let _: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(PersistenceError::Query)?;
        Ok(())
    }

    /// Reads one infrastructure metadata value.
    ///
    /// # Errors
    ///
    /// Returns a query error when the database cannot answer. This includes the
    /// case where the infrastructure migration has not been applied.
    pub async fn infrastructure_value(
        &self,
        key: &str,
    ) -> Result<Option<String>, PersistenceError> {
        let value = sqlx::query_scalar::<_, String>(
            "SELECT value FROM infrastructure_metadata WHERE key = $1",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(PersistenceError::Query)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpStream;

    use super::Database;
    use crate::redact_connection_secrets;

    const UNREACHABLE_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:1/trading";

    #[test]
    fn migration_creates_infrastructure_metadata_only() {
        let sql = include_str!("../migrations/0001_infrastructure_metadata.sql");
        let normalized = sql.to_ascii_lowercase();
        assert!(normalized.contains("create table infrastructure_metadata"));
        for name in [
            "orders",
            "order_events",
            "trades",
            "positions",
            "risk_reservations",
            "broker_accounts",
            "executions",
            "strategies",
        ] {
            let statement = format!("create table {name}");
            assert!(
                !normalized.contains(&statement),
                "migration must not create {name}"
            );
        }
    }

    #[tokio::test]
    async fn connection_failure_does_not_expose_the_password() {
        if TcpStream::connect("127.0.0.1:1").await.is_ok() {
            panic!("127.0.0.1:1 accepted a connection; the redaction test will not use it");
        }

        let database = Database::connect(UNREACHABLE_URL).expect("url should parse");
        let error = database
            .check_ready()
            .await
            .expect_err("the closed port should fail the readiness check");
        let debug = format!("{error:?}");
        let display = format!("{error}");
        for rendered in [&debug, &display] {
            let scrubbed = redact_connection_secrets(rendered, UNREACHABLE_URL);
            assert!(
                !rendered.contains("super-secret-password"),
                "error output included the database password: {scrubbed}"
            );
        }
    }
}
