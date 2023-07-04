use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::server::ServerApp;

pub async fn check_room_exist_handler(
    State(server_app): State<Arc<ServerApp>>,
    Json(room_id): Json<String>,
) -> impl IntoResponse {
    let result = server_app.check_exist_room(&room_id).await;
    (StatusCode::OK, Json(result)).into_response()
}
