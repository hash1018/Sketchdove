use lib::user::{User, UserResponse};
use reqwasm::http;

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    StatusError(u16, String),
    ParseError,
    UserError(UserResponse),
}

pub async fn api_register_user(user: User) -> Result<(), ApiError> {
    let user = serde_json::to_string(&user).unwrap();
    let request = http::Request::post("/api/user/register")
        .header("Content-Type", "application/json")
        .body(user);

    println!("request register  {request:?}");

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    println!("register response {response:?}");

    log::info!("{response:?}");

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

pub async fn api_login_user(user: User) -> Result<(), ApiError> {
    let user = serde_json::to_string(&user).unwrap();
    let request = http::Request::post("/api/user/login")
        .header("Content-Type", "application/json")
        .body(user);

    println!("request login  {request:?}");

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    println!("login response {response:?}");

    log::info!("{response:?}");

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
