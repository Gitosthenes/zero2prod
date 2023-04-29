use crate::routes::{health_check, subscription};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscription))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ok() {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("Test listener failed to bind to random port");

        assert!(run(listener).is_ok())
    }
}
