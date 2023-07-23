use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use tracing::log::info;
use std::env;
use zero_to_production::*;

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

    let my_variable = env::var("APP_DATABASE__HOST").unwrap();
    info!("{:?}",my_variable);

    let config = get_configuration().expect("Failed to read configuration");

    Application::build(&config).await.unwrap().run().await;
}
