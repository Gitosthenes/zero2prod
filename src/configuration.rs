use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn wout_db(&self) -> PgConnectOptions {
        let ssl = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.wout_db().database(&self.database_name);
        options.log_statements(tracing_log::log::LevelFilter::Trace);

        options
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get path to configuration
    let config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("configuration");

    // Detect running environment (default: local)
    let env: Environment =
        Environment::from_str(&std::env::var("APP_ENVIRONMENT").unwrap_or(String::from("local")))
            .unwrap_or(Environment::Local);

    // Environment config file name
    let env_file = format!("{}.yml", env.as_ref());

    // Initialize config
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yml")))
        .add_source(config::File::from(config_path.join(env_file)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}

// Serialized names must match name of .yaml file in /configuration directory
#[derive(AsRefStr, EnumString)]
pub enum Environment {
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "production")]
    Production,
}
