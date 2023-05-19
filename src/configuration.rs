use secrecy::{ExposeSecret, Secret};

pub enum ConnectTo {
    Server,
    Database,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
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
    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(
            // Add configuration values from a file named `configuration.yaml`.
            config::File::new("config.yaml", config::FileFormat::Yaml),
        )
        .build()?;

    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}
