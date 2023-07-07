pub mod subscription;

use axum::http::StatusCode;
use tracing::instrument;

#[utoipa::path(get, path = "/", tag = "basic")]
#[instrument]
pub async fn handler() -> &'static str {
    "Hello, world!"
}

#[utoipa::path(get, path = "/health_check", tag = "basic")]
#[instrument]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
