use lib::message::{RequestType, ResponseType};
use reqwasm::http;

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    ParseError,
}

pub async fn api_check_room_exist(room_id: &str) -> Result<bool, ApiError> {
    let body = serde_json::to_string(&RequestType::CheckRoomExist(room_id.to_string())).unwrap();
    let request = http::Request::post("/api/check_room_exist")
        .header("Content-Type", "application/json")
        .body(body);

    let response = request
        .send()
        .await
        .map_err(|_| ApiError::FailedToSendRequest)?;

    let response = response
        .json::<ResponseType>()
        .await
        .map_err(|_| ApiError::ParseError)?;

    match response {
        ResponseType::ResponseRoomExist(result) => Ok(result),
        _ => unreachable!(),
    }
}

pub async fn api_check_user_exist(user_id: &str, room_id: &str) -> Result<bool, ApiError> {
    let body = serde_json::to_string(&RequestType::CheckUserExist(
        room_id.to_string(),
        user_id.to_string(),
    ))
    .unwrap();
    let request = http::Request::post("/api/check_user_exist")
        .header("Content-Type", "application/json")
        .body(body);

    let response = request
        .send()
        .await
        .map_err(|_| ApiError::FailedToSendRequest)?;

    let response = response
        .json::<ResponseType>()
        .await
        .map_err(|_| ApiError::ParseError)?;

    match response {
        ResponseType::ResponseUserExist(result) => {
            if let Some(result) = result {
                Ok(result)
            } else {
                Err(ApiError::ParseError)
            }
        }
        _ => unreachable!(),
    }
}
