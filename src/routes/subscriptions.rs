use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[instrument(
    name = "Adding new subscriber",
    skip_all,
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    // Parse email from form data
    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(valid_email) => valid_email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    // Parse name from form data
    let name = match SubscriberName::parse(form.0.name) {
        Ok(valid_name) => valid_name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Create and insert new subscriber
    let new_sub = NewSubscriber { email, name };
    match insert_subscriber(&new_sub, &db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[instrument(name = "Saving new subscriber details in the database", skip_all)]
async fn insert_subscriber(
    subscriber: &NewSubscriber,
    db_pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
    })
}
