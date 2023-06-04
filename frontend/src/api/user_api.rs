use lib::user::{User, UserResponse};
use reqwasm::http;

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    StatusError(u16, String),
    ParseError,
    UserError(UserResponse),
}

pub async fn api_register_user(user: User) -> Result<User, ApiError> {
    let user = serde_json::to_string(&user).unwrap();
    let response = match http::Request::post("/api/user/register")
        .header("Content-Type", "application/json")
        .body(user)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    log::info!("{response:?}");

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => match data {
            UserResponse::Registered(user) => Ok(user),
            UserResponse::AlreadyExist(user) => {
                Err(ApiError::UserError(UserResponse::AlreadyExist(user)))
            }
        },
        Err(_) => Err(ApiError::ParseError),
    }
}
