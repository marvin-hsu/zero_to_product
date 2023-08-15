use axum::{
    routing::{get, post},
    Router, Server,
};
use axum::http::HeaderValue;
use jsonwebtoken::{Algorithm, Header};
use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use url::Url;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    confirm, health_check, login, subscribe, ApiDoc, Authorization, DatabaseSettings, EmailClient,
    EmailClientSettings, JwtHandler, JwtHandlerSettings, Settings,
};

pub struct Application {
    port: u16,
    router: Router,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, std::io::Error> {
        let database = get_database(&config.database).await.unwrap();
        let email_client = get_email_client(&config.email_client);
        let jwt_handler = get_jwt_handler(&config.jwt_handler);
        let auth = Authorization {
            jwt_handler: jwt_handler.clone(),
        };

        let router = Router::new()
            .layer(ValidateRequestHeaderLayer::custom(auth))
            .route("/health_check", get(health_check))
            .route("/subscriptions", post(subscribe))
            .route("/subscriptions/confirm/:token", get(confirm))
            .route("/login", post(login))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(CorsLayer::new()
                .allow_credentials(true)
                .allow_origin("http://localhost:5173/".parse::<HeaderValue>().unwrap()))
            .layer(TraceLayer::new_for_http())
            .with_state(AppState {
                database,
                base_url: config.application.base_url.parse().unwrap(),
                email_client,
                jwt_handler,
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

fn get_jwt_handler(settings: &JwtHandlerSettings) -> JwtHandler {
    JwtHandler {
        private_key: settings.private_key.clone(),
        public_key: settings.public_key.clone(),
        header: Header::new(Algorithm::HS512),
        expiration_minutes: settings.expiration_minutes,
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub email_client: EmailClient,
    pub base_url: Url,
    pub jwt_handler: JwtHandler,
}
