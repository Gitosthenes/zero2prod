use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    //Setup application and client
    let addr = spawn_app();
    let client = reqwest::Client::new();

    //Send GET request to health_check endpoint
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Setup IP and initial port
    let ip_addr = "127.0.0.1";
    let port = "0";

    // Retrieve port assigned by OS
    let listener =
        TcpListener::bind(format!("{}:{}", ip_addr, port)).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Create and launch server as a background task (tokio::spawn returns a handle to the spawned future)
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _future = tokio::spawn(server);

    // Return application address to the caller.
    format!("http://{}:{}", ip_addr, port)
}
