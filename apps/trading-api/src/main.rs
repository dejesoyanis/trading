//! Process entry point for the trading API.
//!
//! Trading commands are not served.

use std::process::ExitCode;

use trading_api::{AppError, Config, init_tracing, load_local_env, start};

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
    load_local_env()?;
    let config = Config::from_env()?;
    init_tracing(config.log_level(), config.log_format())?;
    let server = start(config).await?;
    tokio::signal::ctrl_c().await.map_err(AppError::Signal)?;
    tracing::info!("shutdown signal received");
    server.shutdown().await
}
