use axum::http::StatusCode;

use tracing::instrument;

#[utoipa::path(
    get,
    path = "/health_check", 
    tag = "health_check",
    responses(
        (status = 200)
    ))]
#[instrument]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
