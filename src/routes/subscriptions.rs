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
    match insert_subscriber(&pool, &input).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        OffsetDateTime::now_utc()
    )
    .execute(pool)
    .await
    .map_err(|error| {
        tracing::error!("Failed to execute query: {error}");
        error
    })?;
    Ok(())
}
