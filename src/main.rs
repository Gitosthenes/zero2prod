use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::{get_configuration, ConnectTo};
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

    // Get configuration settings and connect to DB
    let config = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&config.database.connection_string(ConnectTo::Database))
        .await
        .expect("Failed to connect to Postgres");

    // Start listening
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind");

    // Start Application
    run(listener, db_pool)?.await
}
