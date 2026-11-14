use std::sync::{Arc, Mutex};

use axum::extract::{Request, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tracing::Instrument;
use trading_common::CorrelationId;
use trading_persistence::{Database, PersistenceError};

use crate::logging::{ReadinessSnapshot, log_readiness_transition};

const SERVICE: &str = "trading-api";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// HTTP readiness is not Trading Readiness and grants no submission authority.
///
/// Dependencies checked by the readiness endpoint. The check is read-only.
#[derive(Clone)]
pub(crate) struct AppState {
    database: Database,
    endpoint_label: String,
    last_readiness: Arc<Mutex<Option<ReadinessSnapshot>>>,
}

impl AppState {
    /// Builds the state shared by health handlers.
    pub(crate) fn new(database: Database, endpoint_label: String) -> Self {
        Self {
            database,
            endpoint_label,
            last_readiness: Arc::new(Mutex::new(None)),
        }
    }

    /// Records a readiness result and logs only when the result changes.
    pub(crate) fn record_result(&self, result: &Result<(), PersistenceError>) {
        let snapshot = match result {
            Ok(()) => ReadinessSnapshot::Ready,
            Err(error) => ReadinessSnapshot::NotReady(error.readiness_problem()),
        };
        let mut guard = self
            .last_readiness
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        log_readiness_transition(&self.endpoint_label, &mut guard, snapshot);
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
/// Only liveness and read-only PostgreSQL readiness are registered.
/// HTTP readiness is not Trading Readiness and grants no submission authority.
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
    let result = state.database.check_ready().await;
    state.record_result(&result);
    match result {
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
        Err(_error) => (
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
            .into_response(),
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
