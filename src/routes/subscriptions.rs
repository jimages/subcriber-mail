use std::ops::Deref;

use actix_web::{web, HttpResponse, Result, error};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[actix_web::post("/subscriptions")]
pub async fn subscriptions(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now().naive_local()
    )
    .execute(connection.as_ref().deref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query {}", e);
        error::ErrorInternalServerError("failed to execute query.")
    })?;
    Ok(HttpResponse::Ok().finish())
}
