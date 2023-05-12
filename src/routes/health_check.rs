use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("Hello Health Check");
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_ok() {
        let res = health_check().await;
        assert!(res.status().is_success());
    }
}
