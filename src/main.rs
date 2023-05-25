use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::time::Duration;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize Global Tracing Subscriber
    init_subscriber(get_subscriber(
        String::from("zero2prod"),
        String::from("info"),
        std::io::stdout,
    ));

    // Get configuration settings
    let config = get_configuration().expect("Failed to read configuration");

    // Connect to DB
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    // Build email client
    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(sender_email, config.email_client.base_url);

    // Start listening
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind");

    // Start Application
    run(listener, db_pool, email_client)?.await
}
