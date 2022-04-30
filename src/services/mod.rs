use axum::response::IntoResponse;
use http::StatusCode;

pub mod notes;

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "WTF!".to_owned())
}
