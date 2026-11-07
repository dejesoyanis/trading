use axum::extract::{Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tracing::Instrument;
use trading_common::error_chain;
use trading_common::{CorrelationId, SecretString};
use trading_persistence::{Database, redact_connection_secrets};

const SERVICE: &str = "trading-api";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Dependencies checked by the readiness endpoint.
#[derive(Clone)]
pub(crate) struct AppState {
    database: Database,
    database_url: SecretString,
    endpoint_label: String,
}

impl AppState {
    /// Builds the state shared by health handlers.
    pub(crate) fn new(
        database: Database,
        database_url: SecretString,
        endpoint_label: String,
    ) -> Self {
        Self {
            database,
            database_url,
            endpoint_label,
        }
    }
}

#[derive(Serialize)]
struct LiveBody {
    service: &'static str,
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ReadyBody {
    service: &'static str,
    status: &'static str,
    version: &'static str,
    checks: ReadyChecks,
}

#[derive(Serialize)]
struct ReadyChecks {
    postgres: &'static str,
}

/// Routes served by the process.
///
/// Only liveness and PostgreSQL readiness are registered.
pub(crate) fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
        .layer(middleware::from_fn(attach_correlation_id))
        .with_state(state)
}

async fn live() -> Json<LiveBody> {
    Json(LiveBody {
        service: SERVICE,
        status: "ok",
        version: VERSION,
    })
}

async fn ready(State(state): State<AppState>) -> Response {
    match state.database.check_ready().await {
        Ok(()) => (
            StatusCode::OK,
            Json(ReadyBody {
                service: SERVICE,
                status: "ready",
                version: VERSION,
                checks: ReadyChecks { postgres: "ok" },
            }),
        )
            .into_response(),
        Err(error) => {
            let message =
                redact_connection_secrets(&error_chain(&error), state.database_url.expose());
            tracing::error!(
                postgres = %state.endpoint_label,
                error = %message,
                "readiness check failed"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ReadyBody {
                    service: SERVICE,
                    status: "not_ready",
                    version: VERSION,
                    checks: ReadyChecks {
                        postgres: "unavailable",
                    },
                }),
            )
                .into_response()
        }
    }
}

async fn attach_correlation_id(request: Request, next: Next) -> Response {
    let correlation_id = CorrelationId::generate();
    let span = tracing::info_span!(
        "http_request",
        correlation_id = %correlation_id,
        method = %request.method(),
        path = %request.uri().path(),
    );
    let mut response = next.run(request).instrument(span).await;
    match HeaderValue::from_str(&correlation_id.to_string()) {
        Ok(value) => {
            response
                .headers_mut()
                .insert(header::HeaderName::from_static("x-correlation-id"), value);
        }
        Err(error) => {
            tracing::error!(
                error = %error,
                "correlation id was not valid as an http header"
            );
        }
    }
    response
}
