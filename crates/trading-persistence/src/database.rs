use std::fmt;
use std::str::FromStr;

use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::endpoint::database_endpoint;
use crate::error::{DatabaseUrlProblem, PersistenceError, ReadinessProblem};
use crate::readiness::{
    ACQUIRE_TIMEOUT, ExpectedMigration, MIGRATION_STATEMENT_TIMEOUT, MIGRATION_TIMEOUT,
    READINESS_STATEMENT_TIMEOUT, READINESS_TIMEOUT, RecordedMigration, assess_metadata,
    assess_migrations, problem_for_sqlstate,
};
use crate::redact::redact_error;

const MIGRATION_HISTORY: &str = "\
SELECT version, checksum, success
FROM _sqlx_migrations
ORDER BY version";

const METADATA_VALUE: &str = "\
SELECT value
FROM infrastructure_metadata
WHERE key = $1";

/// Lazy PostgreSQL pool.
///
/// [`Self::connect`] does not open a socket and does not migrate.
/// [`Self::migrate`] is the only operation that applies infrastructure DDL.
/// [`Self::check_ready`] is read-only.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl fmt::Debug for Database {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Database")
    }
}

impl Database {
    /// Builds a lazy pool from a PostgreSQL URL.
    ///
    /// The pool waits at most [`ACQUIRE_TIMEOUT`] for a connection. Each
    /// statement on a pooled connection uses [`READINESS_STATEMENT_TIMEOUT`].
    /// No migration runs.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::InvalidUrl`] when the URL is rejected.
    /// The error does not include the URL.
    pub fn connect(database_url: &str) -> Result<Self, PersistenceError> {
        let _endpoint = database_endpoint(database_url)?;
        let options = PgConnectOptions::from_str(database_url)
            .map_err(|_| PersistenceError::InvalidUrl(DatabaseUrlProblem::Rejected))?
            .application_name("trading-api")
            .options([("statement_timeout", READINESS_STATEMENT_TIMEOUT)]);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(ACQUIRE_TIMEOUT)
            .connect_lazy_with(options);
        Ok(Self { pool })
    }

    /// Applies embedded infrastructure migrations.
    ///
    /// This is idempotent under SQLx migration rules: a current schema is left
    /// unchanged, and a checksum mismatch is returned rather than repaired.
    /// The connection used here is closed when the call finishes so its longer
    /// statement timeout cannot be reused by readiness.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::Migrate`] when PostgreSQL cannot be reached
    /// or the migration cannot be applied. The detail is redacted.
    pub async fn migrate(&self) -> Result<(), PersistenceError> {
        match tokio::time::timeout(MIGRATION_TIMEOUT, self.migrate_once()).await {
            Ok(result) => result,
            Err(_elapsed) => Err(PersistenceError::Migrate {
                detail: "migration timed out".to_owned(),
            }),
        }
    }

    /// Checks that PostgreSQL accepts a connection.
    ///
    /// This is `SELECT 1` only. It does not decide schema readiness.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::NotReady`] when the database cannot be reached
    /// in time. The error does not include driver text.
    pub async fn check_connection(&self) -> Result<(), PersistenceError> {
        bounded(self.select_one()).await
    }

    /// Checks connectivity and infrastructure schema compatibility.
    ///
    /// The check is read-only. It does not run SQLx migrations or any other
    /// DDL, `INSERT`, `UPDATE`, or `DELETE`. A database that answers `SELECT 1`
    /// and has no compatible migration history is not ready. A failed check
    /// does not repair the schema.
    ///
    /// Compatibility requires every embedded migration to be recorded in
    /// `_sqlx_migrations` with `success = true` and the embedded checksum, and
    /// `infrastructure_metadata.schema_purpose` to equal the infrastructure-only
    /// value. Either fact alone is not enough.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::NotReady`] when the database is unreachable,
    /// the check times out, or the schema is missing or incompatible.
    pub async fn check_ready(&self) -> Result<(), PersistenceError> {
        bounded(self.read_schema()).await
    }

    /// Reads one infrastructure metadata value.
    ///
    /// # Errors
    ///
    /// Returns [`PersistenceError::NotReady`] when the database cannot answer.
    /// That includes a database where the infrastructure migration has not been
    /// applied. The error does not include driver text.
    pub async fn infrastructure_value(
        &self,
        key: &str,
    ) -> Result<Option<String>, PersistenceError> {
        let value = sqlx::query_scalar::<_, String>(METADATA_VALUE)
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| PersistenceError::NotReady(problem_for_sqlx(&error)))?;
        Ok(value)
    }

    async fn migrate_once(&self) -> Result<(), PersistenceError> {
        let mut connection =
            self.pool
                .acquire()
                .await
                .map_err(|error| PersistenceError::Migrate {
                    detail: redact_error(&error),
                })?;
        let result = apply_migrations(&mut connection).await;
        let _ = connection.close().await;
        result
    }

    async fn select_one(&self) -> Result<(), PersistenceError> {
        let _: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|error| PersistenceError::NotReady(problem_for_sqlx(&error)))?;
        Ok(())
    }

    async fn read_schema(&self) -> Result<(), PersistenceError> {
        self.select_one().await?;
        let applied = load_migration_history(&self.pool).await?;
        let expected = expected_migrations();
        assess_migrations(&applied, &expected).map_err(PersistenceError::NotReady)?;
        let metadata = load_metadata(&self.pool).await?;
        assess_metadata(metadata.as_deref()).map_err(PersistenceError::NotReady)
    }
}

async fn apply_migrations(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<(), PersistenceError> {
    let _: String = sqlx::query_scalar("SELECT set_config('statement_timeout', $1, false)")
        .bind(MIGRATION_STATEMENT_TIMEOUT)
        .fetch_one(connection.as_mut())
        .await
        .map_err(|error| PersistenceError::Migrate {
            detail: redact_error(&error),
        })?;
    sqlx::migrate!("./migrations")
        .run(&mut *connection)
        .await
        .map_err(|error| PersistenceError::Migrate {
            detail: redact_error(&error),
        })?;
    Ok(())
}

fn expected_migrations() -> Vec<ExpectedMigration> {
    sqlx::migrate!("./migrations")
        .iter()
        .map(|migration| ExpectedMigration {
            version: migration.version,
            checksum: migration.checksum.clone().into_owned(),
        })
        .collect()
}

async fn load_migration_history(pool: &PgPool) -> Result<Vec<RecordedMigration>, PersistenceError> {
    match sqlx::query_as::<_, (i64, Vec<u8>, bool)>(MIGRATION_HISTORY)
        .fetch_all(pool)
        .await
    {
        Ok(rows) => Ok(rows
            .into_iter()
            .map(|(version, checksum, success)| RecordedMigration {
                version,
                checksum,
                success,
            })
            .collect()),
        Err(error) => {
            let problem = problem_for_sqlx(&error);
            if problem == ReadinessProblem::SchemaMissing {
                Ok(Vec::new())
            } else {
                Err(PersistenceError::NotReady(problem))
            }
        }
    }
}

async fn load_metadata(pool: &PgPool) -> Result<Option<String>, PersistenceError> {
    use crate::readiness::INFRASTRUCTURE_METADATA_KEY;

    match sqlx::query_scalar::<_, String>(METADATA_VALUE)
        .bind(INFRASTRUCTURE_METADATA_KEY)
        .fetch_optional(pool)
        .await
    {
        Ok(value) => Ok(value),
        Err(error) => {
            let problem = match problem_for_sqlx(&error) {
                ReadinessProblem::SchemaMissing => ReadinessProblem::SchemaIncompatible,
                other => other,
            };
            Err(PersistenceError::NotReady(problem))
        }
    }
}

async fn bounded(
    check: impl std::future::Future<Output = Result<(), PersistenceError>>,
) -> Result<(), PersistenceError> {
    match tokio::time::timeout(READINESS_TIMEOUT, check).await {
        Ok(result) => result,
        Err(_elapsed) => Err(PersistenceError::NotReady(ReadinessProblem::TimedOut)),
    }
}

fn problem_for_sqlx(error: &sqlx::Error) -> ReadinessProblem {
    match error {
        sqlx::Error::PoolTimedOut => ReadinessProblem::TimedOut,
        sqlx::Error::Database(database_error) => {
            problem_for_sqlstate(database_error.code().as_deref())
        }
        _ => ReadinessProblem::Unreachable,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::net::TcpStream;

    use super::Database;
    use crate::error::{PersistenceError, ReadinessProblem};
    use crate::readiness::{INFRASTRUCTURE_METADATA_KEY, INFRASTRUCTURE_METADATA_VALUE};

    const UNREACHABLE_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:1/trading";

    #[test]
    fn migration_creates_infrastructure_metadata_only() {
        let sql = include_str!("../migrations/0001_infrastructure_metadata.sql");
        let normalized = sql.to_ascii_lowercase();
        assert!(normalized.contains("create table infrastructure_metadata"));
        assert!(normalized.contains(&format!(
            "'{INFRASTRUCTURE_METADATA_KEY}', '{INFRASTRUCTURE_METADATA_VALUE}'"
        )));
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
        let started = std::time::Instant::now();
        let error = database
            .check_ready()
            .await
            .expect_err("the closed port should fail the readiness check");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "readiness exceeded its bound"
        );
        assert!(
            matches!(
                error,
                PersistenceError::NotReady(
                    ReadinessProblem::Unreachable | ReadinessProblem::TimedOut
                )
            ),
            "{error:?}"
        );
        assert_secret_free(&error);

        let migrate_error = database
            .migrate()
            .await
            .expect_err("migrate should fail when postgres is unreachable");
        assert!(matches!(migrate_error, PersistenceError::Migrate { .. }));
        assert_secret_free(&migrate_error);
    }

    fn assert_secret_free(error: &PersistenceError) {
        let rendered = format!("{error} {error:?} {}", error.redacted_detail());
        assert!(
            !rendered.contains("super-secret-password"),
            "error output included the database password: {rendered}"
        );
        assert!(
            !rendered.contains(UNREACHABLE_URL),
            "error output included the database url: {rendered}"
        );
    }
}
