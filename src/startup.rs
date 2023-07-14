use axum::{
    routing::{get, post},
    Router, Server,
};
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{health_check, subscribe, ApiDoc, DatabaseSettings, Settings};

pub struct Application {
    port: u16,
    router: Router,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, std::io::Error> {
        let database = get_database(&config.database).await.unwrap();

        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .route("/health_check", get(health_check))
            .route("/subscriptions", post(subscribe))
            .layer(TraceLayer::new_for_http())
            .with_state(AppState { database });

        Ok(Self {
            port: config.application.port,
            router,
        })
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));

        Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}

pub async fn get_database(
    settings: &DatabaseSettings,
) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(settings.connection_string().expose_secret()).await
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DatabaseConnection,
}
