use tracing_subscriber::util::SubscriberInitExt;
use zero_to_production::{get_configuration, get_subscriber, Application};

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration");

    get_subscriber(&config).init();

    Application::build(&config).await.unwrap().run().await;
}
