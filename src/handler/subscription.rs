use axum::http::StatusCode;
use tracing::instrument;

#[utoipa::path(post, path = "/subscribe", tag = "subscription")]
#[instrument]
pub async fn subscribe()-> StatusCode {
    StatusCode::OK
}