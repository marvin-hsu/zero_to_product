use std::net::SocketAddr;

use secrecy::ExposeSecret;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use zero_to_production::*;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sea_orm::Database;
use tower_http::trace::TraceLayer;

use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "zero_to_production=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = get_configuration().expect("Failed to read configuration");

    let conn = Database::connect(config.database.connection_string().expose_secret())
        .await
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], config.application.port));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(Arc::new(AppState { conn })))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
