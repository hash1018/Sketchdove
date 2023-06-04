use axum::{http::StatusCode, response::IntoResponse, Json};
use lib::user::{User, UserResponse};
use tracing::log;

pub async fn user_register_handler(Json(payload): Json<User>) -> impl IntoResponse {
    log::info!("{payload:?}");
    (StatusCode::OK, Json(UserResponse::Registered(payload))).into_response()
}
