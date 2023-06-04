use lib::user::User;
use reqwasm::http;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    StatusError(u16, String),
    ParseError,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserResponse {
    status: String,
    user: User,
}

pub async fn api_register_user(user: User) -> Result<User, ApiError> {
    let user = serde_json::to_string(&user).unwrap();
    let response = match http::Request::post("/api/auth/register")
        .header("Content-Type", "application/json")
        .body(user)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let status = response.status();

    if status != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(ApiError::StatusError(status, error_response.message));
        } else {
            return Err(ApiError::StatusError(status, "".to_string()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.user),
        Err(_) => Err(ApiError::ParseError),
    }
}
