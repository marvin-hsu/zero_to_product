use axum::http::StatusCode;
use tracing::instrument;

#[utoipa::path(get, path = "/")]
#[instrument]
pub async fn handler() -> &'static str {
    "Hello, world!"
}

#[utoipa::path(get, path = "/health_check")]
#[instrument]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
