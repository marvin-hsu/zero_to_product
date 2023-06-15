use axum::{http::StatusCode, routing::get, Router};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(api::handler, api::health_check))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(api::handler))
        .route("/health_check", get(api::health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod api {
    use super::*;
    #[utoipa::path(get, path = "/")]
    pub async fn handler() -> &'static str {
        "Hello, world!"
    }

    #[utoipa::path(get, path = "/health_check")]
    pub async fn health_check() -> StatusCode {
        StatusCode::OK
    }
}
