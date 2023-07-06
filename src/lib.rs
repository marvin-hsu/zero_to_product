pub mod doc;
pub mod handler;

use axum::{routing::get, Router};
use doc::ApiDoc;
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn app() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(handler::handler))
        .route("/health_check", get(handler::health_check))
        .layer(TraceLayer::new_for_http())
}
