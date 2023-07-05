pub mod doc;

use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get, Router};
use doc::ApiDoc;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "zero_to_production=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(handler))
        .route("/health_check", get(health_check))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

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
