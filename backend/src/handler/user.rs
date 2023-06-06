use axum::{http::StatusCode, response::IntoResponse, Json};
use lib::user::{User, UserResponse};
use tower_cookies::{Cookie, Cookies};
use tracing::log;

pub async fn user_register_handler(
    cookies: Cookies,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get("auth-token") {
        log::info!("cookie exist {cookie:?}");
    } else {
        log::info!("cookie does not exist");
    }
    (StatusCode::OK, Json(UserResponse::Registered)).into_response()
}

pub async fn user_login_handler(cookies: Cookies, Json(_payload): Json<User>) -> impl IntoResponse {
    cookies.add(Cookie::new("auth-token", "user-1.exp.sign"));
    (StatusCode::OK, Json(UserResponse::LogedIn)).into_response()
}

pub async fn user_logout_handler(
    cookies: Cookies,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    cookies.remove(Cookie::new("auth-token", ""));
    (StatusCode::OK, Json(UserResponse::LogedOut)).into_response()
}
