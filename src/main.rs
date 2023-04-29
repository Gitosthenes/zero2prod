use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Retrieve port assigned by OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");

    dbg!(listener
        .local_addr()
        .expect("Failed to retrieve port.")
        .port());

    // Start listening
    run(listener)?.await
}
