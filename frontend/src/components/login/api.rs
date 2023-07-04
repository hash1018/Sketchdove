use reqwasm::http;

#[derive(Debug)]
pub enum ApiError {
    FailedToSendRequest,
    ParseError,
}

pub async fn api_check_room_exist(room_id: &str) -> Result<bool, ApiError> {
    let body = serde_json::to_string(room_id).unwrap();
    let request = http::Request::post("/api/check_room_exist")
        .header("Content-Type", "application/json")
        .body(body);

    let result = request.send().await;

    let response = match result {
        Ok(res) => res,
        Err(_) => return Err(ApiError::FailedToSendRequest),
    };

    let response = response.json::<bool>().await;

    match response {
        Ok(result) => Ok(result),
        Err(_) => Err(ApiError::ParseError),
    }
}
