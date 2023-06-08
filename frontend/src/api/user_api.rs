use lib::user::{User, UserResponse};
use reqwasm::http;

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    StatusError(u16, String),
    ParseError,
    UserError(UserResponse),
}

pub async fn api_register_user(user: &User) -> Result<(), ApiError> {
    let user = serde_json::to_string(user).unwrap();
    let request = http::Request::post("/api/user/register")
        .header("Content-Type", "application/json")
        .body(user);

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => match data {
            UserResponse::Registered => Ok(()),
            UserResponse::AlreadyExist => Err(ApiError::UserError(UserResponse::AlreadyExist)),
            _ => unreachable!(),
        },
        Err(_) => Err(ApiError::ParseError),
    }
}

pub async fn api_login_user(user: &User) -> Result<(), ApiError> {
    let user = serde_json::to_string(user).unwrap();
    let request = http::Request::post("/api/user/login")
        .header("Content-Type", "application/json")
        .body(user);

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => match data {
            UserResponse::LogedIn => Ok(()),
            UserResponse::DoesNotExist => Err(ApiError::UserError(UserResponse::DoesNotExist)),
            UserResponse::LoginFailed => Err(ApiError::UserError(UserResponse::LoginFailed)),
            _ => unreachable!(),
        },
        Err(_) => Err(ApiError::ParseError),
    }
}

pub async fn api_logout_user(user: &User) -> Result<(), ApiError> {
    let user = serde_json::to_string(user).unwrap();
    let request = http::Request::post("/api/user/logout")
        .header("Content-Type", "application/json")
        .body(user);

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => match data {
            UserResponse::LogedOut => Ok(()),
            UserResponse::DoesNotExist => Err(ApiError::UserError(UserResponse::DoesNotExist)),
            _ => unreachable!(),
        },
        Err(_) => Err(ApiError::ParseError),
    }
}

pub async fn api_check_login_valid() -> Result<(), ApiError> {
    let request = http::Request::get("/api/user/valid").header("Content-Type", "application/json");

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => match data {
            UserResponse::LoginValied => Ok(()),
            UserResponse::LoginExpired => Err(ApiError::UserError(UserResponse::LoginExpired)),
            _ => unreachable!(),
        },
        Err(_) => Err(ApiError::ParseError),
    }
}
