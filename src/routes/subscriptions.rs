use axum::{extract::Form, http::StatusCode};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(_input): Form<FormData>) -> StatusCode {
    StatusCode::OK
}
