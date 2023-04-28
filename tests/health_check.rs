use std::net::TcpListener;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Setup
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let post_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    //Action
    let response = client
        .post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .expect("Failed to execute POST");

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Setup
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    //Action
    for (post_body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_body)
            .send()
            .await
            .expect("Failed to execute POST.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        )
    }
}

#[tokio::test]
async fn health_check_works() {
    //Setup
    let addr = spawn_app();
    let client = reqwest::Client::new();

    //Action
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
