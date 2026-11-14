//! PostgreSQL integration tests.
//!
//! `cargo test --workspace` does not run this file. GitHub Actions runs it with
//! `--ignored` against a PostgreSQL service. A developer database is not used:
//! the test reads `TRADING_TEST_DATABASE_URL` only after
//! `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1`, and the database name must
//! contain `_test`. `DATABASE_URL` is ignored.
//!
//! Destructive steps run against a uniquely named database created for the
//! test and dropped afterward. They do not migrate or alter the opted-in
//! database itself.
//!
//! A full PostgreSQL process restart is not covered. Connection recovery is
//! tested by terminating other backends of the isolated database.

use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::postgres::PgPool;
use sqlx::{AssertSqlSafe, raw_sql};
use trading_persistence::{
    Database, INFRASTRUCTURE_METADATA_KEY, INFRASTRUCTURE_METADATA_VALUE, PersistenceError,
    ReadinessProblem,
};
use url::Url;

#[tokio::test]
#[ignore = "requires TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1 and TRADING_TEST_DATABASE_URL"]
async fn migration_is_explicit_and_readiness_is_read_only() {
    let admin_url = test_database_url();
    let admin = PgPool::connect(&admin_url)
        .await
        .expect("opt-in test database should accept a connection");
    let names = [
        unique_name(0),
        unique_name(1),
        unique_name(2),
        unique_name(3),
    ];
    for name in &names {
        create_database(&admin, name)
            .await
            .unwrap_or_else(|_| panic!("failed to create isolated test database {name}"));
    }

    let admin_url_for_thread = admin_url.clone();
    let names_for_thread = names.clone();
    // A dedicated thread avoids tokio::spawn's Send bound. Async methods that
    // borrow a pool are not always proven Send, and this test only needs the
    // work to finish or panic before the databases are dropped.
    let outcome = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("integration test runtime");
        runtime.block_on(async move {
            fresh_database_becomes_ready_without_readiness_writes(
                admin_url_for_thread.clone(),
                names_for_thread[0].clone(),
            )
            .await;
            deleted_metadata_is_not_repaired(
                admin_url_for_thread.clone(),
                names_for_thread[1].clone(),
            )
            .await;
            checksum_mismatch_is_not_repaired(
                admin_url_for_thread.clone(),
                names_for_thread[2].clone(),
            )
            .await;
            unknown_migration_version_is_not_repaired(
                admin_url_for_thread,
                names_for_thread[3].clone(),
            )
            .await;
        });
    })
    .join();

    for name in &names {
        if drop_database(&admin, name).await.is_err() {
            panic!("failed to drop isolated test database {name}");
        }
    }
    admin.close().await;

    if let Err(panic) = outcome {
        std::panic::resume_unwind(panic);
    }
}

async fn fresh_database_becomes_ready_without_readiness_writes(admin_url: String, name: String) {
    let url = database_url(&admin_url, &name);
    let database = Database::connect(&url).expect("isolated database url should parse");

    database
        .check_connection()
        .await
        .expect("a fresh database should accept SELECT 1");
    let missing = database
        .check_ready()
        .await
        .expect_err("a fresh database should not be ready");
    assert_eq!(missing.public_reason(), "schema_missing");
    assert!(matches!(
        missing,
        PersistenceError::NotReady(ReadinessProblem::SchemaMissing)
    ));
    assert_secret_free(&missing);

    let pool = PgPool::connect(&url)
        .await
        .expect("inspection pool should connect");
    assert!(
        public_tables(pool.clone()).await.is_empty(),
        "a fresh database should not already contain infrastructure tables"
    );

    database
        .migrate()
        .await
        .expect("explicit migration should apply the infrastructure schema");
    database
        .check_ready()
        .await
        .expect("the same pool should become ready after an explicit migration");
    let purpose = database
        .infrastructure_value(INFRASTRUCTURE_METADATA_KEY)
        .await
        .expect("metadata should be readable");
    assert_eq!(purpose.as_deref(), Some(INFRASTRUCTURE_METADATA_VALUE));

    let before = snapshot(pool.clone()).await;
    assert_eq!(
        before.tables,
        vec![
            "_sqlx_migrations".to_owned(),
            "infrastructure_metadata".to_owned(),
        ]
    );
    for forbidden in [
        "orders",
        "order_events",
        "trades",
        "positions",
        "risk_reservations",
        "broker_accounts",
        "executions",
        "strategies",
    ] {
        assert!(
            !before.tables.iter().any(|table| table == forbidden),
            "migration created a trading table: {forbidden}"
        );
    }

    for _ in 0..5 {
        database
            .check_ready()
            .await
            .expect("repeated readiness should stay successful");
    }
    assert_eq!(
        snapshot(pool.clone()).await,
        before,
        "readiness checks must not change migration history or public tables"
    );

    database
        .migrate()
        .await
        .expect("a second migration should be idempotent");
    assert_eq!(
        snapshot(pool.clone()).await.rows,
        before.rows,
        "an idempotent migration must not rewrite history"
    );

    let _: Vec<bool> = sqlx::query_scalar(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()",
    )
    .bind(name.clone())
    .fetch_all(&pool)
    .await
    .expect("terminating pooled backends should be allowed for the test role");

    let mut recovered = false;
    for _ in 0..3 {
        if database.check_ready().await.is_ok() {
            recovered = true;
            break;
        }
    }
    assert!(
        recovered,
        "readiness should recover after pooled connections are closed"
    );
    assert_eq!(
        snapshot(pool.clone()).await.rows,
        before.rows,
        "connection recovery must not migrate or rewrite history"
    );
    pool.close().await;
}

async fn deleted_metadata_is_not_repaired(admin_url: String, name: String) {
    let url = database_url(&admin_url, &name);
    let database = Database::connect(&url).expect("isolated database url should parse");
    database.migrate().await.expect("migrate");
    database
        .check_ready()
        .await
        .expect("ready before tampering");

    let pool = PgPool::connect(&url).await.expect("inspection pool");
    let history = migration_rows(pool.clone()).await;
    sqlx::query("DELETE FROM infrastructure_metadata WHERE key = $1")
        .bind(INFRASTRUCTURE_METADATA_KEY)
        .execute(&pool)
        .await
        .expect("delete metadata");

    let error = database
        .check_ready()
        .await
        .expect_err("deleted metadata should not be ready");
    assert_eq!(
        error.readiness_problem(),
        ReadinessProblem::SchemaIncompatible
    );
    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM infrastructure_metadata")
        .fetch_one(&pool)
        .await
        .expect("count metadata");
    assert_eq!(remaining, 0, "readiness must not restore deleted metadata");
    assert_eq!(migration_rows(pool.clone()).await, history);
    pool.close().await;
}

async fn checksum_mismatch_is_not_repaired(admin_url: String, name: String) {
    let url = database_url(&admin_url, &name);
    let database = Database::connect(&url).expect("isolated database url should parse");
    database.migrate().await.expect("migrate");

    let pool = PgPool::connect(&url).await.expect("inspection pool");
    let forged = vec![0xDE_u8, 0xAD];
    sqlx::query("UPDATE _sqlx_migrations SET checksum = $1 WHERE version = 1")
        .bind(&forged)
        .execute(&pool)
        .await
        .expect("corrupt checksum");

    let error = database
        .check_ready()
        .await
        .expect_err("a mismatched checksum should not be ready");
    assert_eq!(
        error.readiness_problem(),
        ReadinessProblem::SchemaIncompatible
    );
    assert_checksum(pool.clone(), forged.clone()).await;

    let migrate_error = database
        .migrate()
        .await
        .expect_err("migration must not repair a checksum mismatch");
    assert!(matches!(migrate_error, PersistenceError::Migrate { .. }));
    assert_secret_free(&migrate_error);
    assert_checksum(pool.clone(), forged.clone()).await;
    pool.close().await;
}

async fn unknown_migration_version_is_not_repaired(admin_url: String, name: String) {
    let url = database_url(&admin_url, &name);
    let database = Database::connect(&url).expect("isolated database url should parse");
    database.migrate().await.expect("migrate");

    let pool = PgPool::connect(&url).await.expect("inspection pool");
    sqlx::query(
        "INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) VALUES ($1, $2, TRUE, $3, 0)",
    )
    .bind(9_999_i64)
    .bind("forged")
    .bind(vec![0_u8])
    .execute(&pool)
    .await
    .expect("insert unknown migration version");

    let error = database
        .check_ready()
        .await
        .expect_err("an unknown migration version should not be ready");
    assert_eq!(
        error.readiness_problem(),
        ReadinessProblem::SchemaIncompatible
    );
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations WHERE version = 9999")
            .fetch_one(&pool)
            .await
            .expect("count forged version");
    assert_eq!(
        count, 1,
        "readiness must not delete an unknown migration row"
    );
    pool.close().await;
}

fn test_database_url() -> String {
    match std::env::var("TRADING_ALLOW_DATABASE_INTEGRATION_TESTS") {
        Ok(value) if value == "1" => {}
        _ => panic!(
            "set TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1 to run database integration tests"
        ),
    }
    if std::env::var_os("TRADING_TEST_DATABASE_URL").is_none()
        && std::env::var_os("DATABASE_URL").is_some()
    {
        panic!(
            "DATABASE_URL is ignored. Set TRADING_TEST_DATABASE_URL to a disposable database whose name contains _test"
        );
    }
    let url = std::env::var("TRADING_TEST_DATABASE_URL").unwrap_or_else(|_| {
        panic!("set TRADING_TEST_DATABASE_URL to an isolated database whose name contains _test")
    });
    let parsed = Url::parse(&url).unwrap_or_else(|_| {
        panic!("TRADING_TEST_DATABASE_URL is not a URL; the value was not printed")
    });
    if !matches!(parsed.scheme(), "postgres" | "postgresql") {
        panic!("TRADING_TEST_DATABASE_URL must use the postgres or postgresql scheme");
    }
    let name = parsed.path().trim_start_matches('/');
    if !name.contains("_test") {
        panic!(
            "TRADING_TEST_DATABASE_URL database name must contain _test so this test cannot target an unrelated database"
        );
    }
    url
}

fn unique_name(index: u8) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the unix epoch")
        .as_nanos();
    let name = format!("trading_test_{index}_{nanos}");
    safe_test_ident(&name).to_owned()
}

fn database_url(admin_url: &str, name: &str) -> String {
    let name = safe_test_ident(name);
    let mut parsed = Url::parse(admin_url).expect("admin url should already have been validated");
    parsed.set_path(&format!("/{name}"));
    parsed.to_string()
}

fn safe_test_ident(name: &str) -> &str {
    assert!(
        !name.is_empty()
            && name.len() <= 63
            && name.contains("_test")
            && name.chars().all(|character| character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'),
        "refusing to interpolate an unsafe database name"
    );
    name
}

async fn create_database(admin: &PgPool, name: &str) -> Result<(), sqlx::Error> {
    let name = safe_test_ident(name);
    let sql = format!("CREATE DATABASE \"{name}\"");
    raw_sql(AssertSqlSafe(sql)).execute(admin).await?;
    Ok(())
}

async fn drop_database(admin: &PgPool, name: &str) -> Result<(), sqlx::Error> {
    let name = safe_test_ident(name);
    let sql = format!("DROP DATABASE IF EXISTS \"{name}\" WITH (FORCE)");
    raw_sql(AssertSqlSafe(sql)).execute(admin).await?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Snapshot {
    rows: Vec<(i64, Vec<u8>, bool, i64)>,
    tables: Vec<String>,
}

async fn snapshot(pool: PgPool) -> Snapshot {
    Snapshot {
        rows: migration_rows(pool.clone()).await,
        tables: public_tables(pool).await,
    }
}

async fn migration_rows(pool: PgPool) -> Vec<(i64, Vec<u8>, bool, i64)> {
    sqlx::query_as(
        "SELECT version, checksum, success, execution_time FROM _sqlx_migrations ORDER BY version",
    )
    .fetch_all(&pool)
    .await
    .expect("migration history should be readable")
}

async fn public_tables(pool: PgPool) -> Vec<String> {
    let mut tables: Vec<String> = sqlx::query_scalar(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name",
    )
    .fetch_all(&pool)
    .await
    .expect("table list should be readable");
    tables.sort();
    tables
}

async fn assert_checksum(pool: PgPool, expected: Vec<u8>) {
    let checksum: Vec<u8> =
        sqlx::query_scalar("SELECT checksum FROM _sqlx_migrations WHERE version = 1")
            .fetch_one(&pool)
            .await
            .expect("checksum should still be readable");
    assert_eq!(checksum, expected);
}

fn assert_secret_free(error: &PersistenceError) {
    let rendered = format!("{error} {error:?} {}", error.redacted_detail());
    assert!(!rendered.contains("postgres://"), "{rendered}");
    assert!(!rendered.contains("postgresql://"), "{rendered}");
}
