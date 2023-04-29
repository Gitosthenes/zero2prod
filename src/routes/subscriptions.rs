use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscription(form: web::Form<FormData>) -> HttpResponse {
    println!("Hello {} => {}", form.name, form.email);
    HttpResponse::Ok().finish()
}
