use axum::{
    routing::{get, post},
    Router, Server,
};
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    health_check, subscribe, ApiDoc, DatabaseSettings, EmailClient, EmailClientSettings, Settings,
};

pub struct Application {
    port: u16,
    router: Router,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, std::io::Error> {
        let database = get_database(&config.database).await.unwrap();
        let email_client = get_email_client(&config.email_client);

        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .route("/health_check", get(health_check))
            .route("/subscriptions", post(subscribe))
            .layer(TraceLayer::new_for_http())
            .with_state(AppState {
                database,
                base_url: config.application.base_url.clone(),
                email_client
            });

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

fn get_email_client(setting: &EmailClientSettings) -> EmailClient {
    let sender_email = setting.sender().expect("Invalid sender email address.");
    let timeout = setting.timeout();

    EmailClient::new(
        setting.base_url.clone(),
        sender_email,
        setting.api_key.clone(),
        timeout,
    )
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub email_client: EmailClient,
    pub base_url: String,
}
