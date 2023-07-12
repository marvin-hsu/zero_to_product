use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use doc::ApiDoc;
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{doc, handler};

pub async fn app() -> Router {
    let connection_string = "postgres://postgres:postgres@localhost:5432/zero_to_production";

    let conn = Database::connect(connection_string).await.unwrap();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health_check", get(handler::health_check))
        .route("/subscriptions", post(handler::subscribe))
        .layer(Extension(Arc::new(AppState { conn })))
        .layer(TraceLayer::new_for_http())
}

#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
