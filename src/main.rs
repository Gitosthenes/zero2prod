use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");

    // Start listening
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind");

    run(listener)?.await
}
