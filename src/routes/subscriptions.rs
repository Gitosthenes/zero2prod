use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Inserting new subscriber.",
        %request_id,
        subscriber_name = %form.name,
        subscriber_email = %form.email
    );
    let _request_span_guard = request_span.enter();

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "[{}] - Successfully inserted new subscriber: '{}', '{}'",
                request_id,
                form.name,
                form.email
            );

            HttpResponse::Ok().finish()
        }
        Err(err) => {
            tracing::error!(
                "[{}] - Failed to insert subscriber: '{}', '{}'\nerr:{:?}",
                request_id,
                form.name,
                form.email,
                err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
