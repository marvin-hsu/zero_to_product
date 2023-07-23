use config::{Config, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    // pub email_client: EmailClientSettings,
}

// pub enum Environment {
//     Local,
//     Production,
// }

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    // pub host: String,
    // pub base_url: String,
}

// #[derive(Deserialize, Clone)]
// pub struct EmailClientSettings {
//     pub base_url: String,
//     pub sender_email: String,
//     pub bear_token: Secret<String>,
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub timeout_milliseconds: u64,
// }

// impl TryFrom<String> for Environment {
//     type Error = String;
//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         match s.to_lowercase().as_str() {
//             "local" => Ok(Self::Local),
//             "production" => Ok(Self::Production),
//             other => Err(format!(
//                 "{} is not a supported environment. Use either `local` or `production`.",
//                 other
//             )),
//         }
//     }
// }

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let mut builder = Config::builder()
        .add_source(File::from(configuration_directory.join("base")).required(true));

    builder = builder.add_source(config::Environment::with_prefix("app").separator("__"));

    let settings = builder.build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
