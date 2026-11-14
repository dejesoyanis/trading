//! Process entry point for the trading API.
//!
//! `trading-api` serves health endpoints. `trading-api migrate` applies
//! infrastructure migrations and exits. Starting the server does not migrate.
//! Trading commands are not served. HTTP readiness is not Trading Readiness
//! and grants no submission authority.

use std::process::ExitCode;

use trading_api::{
    AppError, Command, Config, init_tracing, load_local_env, parse_args, run_until_shutdown, start,
};
use trading_persistence::{Database, database_endpoint, redact_connection_secrets};

fn main() -> ExitCode {
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("trading-api failed: could not start the async runtime: {error}");
            return ExitCode::FAILURE;
        }
    };

    match runtime.block_on(run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("trading-api failed: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), AppError> {
    let command = parse_args(std::env::args())?;
    load_local_env()?;
    let config = Config::from_env()?;
    init_tracing(config.log_level(), config.log_format())?;
    match command {
        Command::Serve => {
            let server = start(config).await?;
            run_until_shutdown(server).await
        }
        Command::Migrate => apply_migrations(&config).await,
    }
}

async fn apply_migrations(config: &Config) -> Result<(), AppError> {
    let endpoint = database_endpoint(config.database_url().expose())?;
    tracing::info!(postgres = %endpoint, "applying infrastructure migrations");
    let database = Database::connect(config.database_url().expose())?;
    match database.migrate().await {
        Ok(()) => {
            tracing::info!(postgres = %endpoint, "infrastructure migrations applied");
            Ok(())
        }
        Err(error) => {
            let detail =
                redact_connection_secrets(&error.redacted_detail(), config.database_url().expose());
            tracing::warn!(
                postgres = %endpoint,
                reason = error.public_reason(),
                detail = %detail,
                "infrastructure migration failed"
            );
            Err(AppError::Database(error))
        }
    }
}
