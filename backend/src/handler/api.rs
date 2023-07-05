use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lib::message::{RequestType, ResponseType};
use std::sync::Arc;

use crate::server::ServerApp;

pub async fn check_room_exist_handler(
    State(server_app): State<Arc<ServerApp>>,
    Json(request_type): Json<RequestType>,
) -> impl IntoResponse {
    match request_type {
        RequestType::CheckRoomExist(room_id) => {
            let result = server_app.check_exist_room(&room_id).await;
            (
                StatusCode::OK,
                Json(ResponseType::ResponseRoomExist(result)),
            )
                .into_response()
        }
        _ => (
            StatusCode::OK,
            Json(ResponseType::InvalidRequest(request_type)),
        )
            .into_response(),
    }
}

pub async fn check_user_exist_handler(
    State(server_app): State<Arc<ServerApp>>,
    Json(request_type): Json<RequestType>,
) -> impl IntoResponse {
    match request_type {
        RequestType::CheckUserExist(room_id, user_id) => {
            match server_app.check_exist_user(&room_id, &user_id).await {
                Ok(result) => (
                    StatusCode::OK,
                    Json(ResponseType::ResponseUserExist(Some(result))),
                )
                    .into_response(),
                Err(_err) => {
                    (StatusCode::OK, Json(ResponseType::ResponseUserExist(None))).into_response()
                }
            }
        }
        _ => (
            StatusCode::OK,
            Json(ResponseType::InvalidRequest(request_type)),
        )
            .into_response(),
    }
}
