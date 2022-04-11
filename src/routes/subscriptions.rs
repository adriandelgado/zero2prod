use axum::{extract::Form, http::StatusCode, Extension};
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(input, pool),
    fields(
        subscriber_email = %input.email,
        subscriber_name = %input.name,
    )
)]
pub async fn subscribe(
    Form(input): Form<FormData>,
    Extension(pool): Extension<PgPool>,
) -> StatusCode {
    tracing::info!("Saving new subscriber details into database.");

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        input.email,
        input.name,
        OffsetDateTime::now_utc()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(error) => {
            tracing::error!("Failed to execute query: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
