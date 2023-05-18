use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, ConnectTo, DatabaseSettings};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test successful POST
    let post_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .expect("Failed to execute POST");

    assert_eq!(200, response.status().as_u16());

    // Test successful INSERT
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    //Action
    for (post_body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscribe", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_body)
            .send()
            .await
            .expect("Failed to execute POST");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request (as expected) when the payload was {}",
            error_msg
        )
    }
}

#[tokio::test]
async fn health_check_works() {
    //Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Action
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // We cannot assign the output of `get_subscriber` to a variable based on the
    // value TEST_LOG` because the sink is part of the type returned by
    // `get_subscriber`, therefore they are not the same type. We could work around
    // it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Get address
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Get DB pool
    let mut config = get_configuration().expect("Failed to read configuration");
    let db_pool = configure_database(&mut config.database).await;

    // Create and launch server as a background task (tokio::spawn returns a handle to the spawned future)
    let server =
        zero2prod::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
    let _future = tokio::spawn(server);

    // Return application
    TestApp { address, db_pool }
}

pub async fn configure_database(config: &mut DatabaseSettings) -> PgPool {
    // Connect to Postgres
    let mut connection = PgConnection::connect(&config.connection_string(ConnectTo::Server))
        .await
        .expect("Failed to connect to Postgres");

    // Create dummy database for test
    config.database_name = Uuid::new_v4().to_string();
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Create connection pool for dummy db
    let connection_pool = PgPool::connect(&config.connection_string(ConnectTo::Database))
        .await
        .expect("Failed to connect to Postgres.");

    // Migrate database
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
