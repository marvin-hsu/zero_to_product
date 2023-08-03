use crate::Settings;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn get_subscriber(setting: &Settings) -> impl SubscriberInitExt {
    let filter = get_filter(&setting.application.logging_levels);

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json())
}

fn get_filter(modules: &[String]) -> EnvFilter {
    let filter_settings = modules.join(",");

    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse_lossy(filter_settings)
}
