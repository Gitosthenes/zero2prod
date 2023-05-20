use secrecy::{ExposeSecret, Secret};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub database_name: String,
    pub host: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub fn connection_string(&self, connect_to: ConnectTo) -> Secret<String> {
        // Append "/db_name" according to `include_name` param
        let db_name = match connect_to {
            ConnectTo::Server => String::new(),
            ConnectTo::Database => format!("/{}", self.database_name),
        };

        // Format/Return connection string as `Secret`
        Secret::new(format!(
            "postgres://{}:{}@{}:{}{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            db_name
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Get path to configuration
    let config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("configuration");

    // Detect running environment (default: local)
    let env: Environment = Environment::from_str(
        std::env::var("APP_ENVIRONMENT")
            .unwrap_or(String::from("local"))
            .as_str(),
    )
    .unwrap_or(Environment::Local);
    // Environment config file name
    let env_file = format!("{}.yml", env.as_ref());

    // Initialize config
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yml")))
        .add_source(config::File::from(config_path.join(env_file)))
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

pub enum ConnectTo {
    Server,
    Database,
}
