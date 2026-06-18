use axum::{response::IntoResponse, Json};
use axum::http::StatusCode;




pub async fn get_runsession_handler() -> impl IntoResponse {

    // they give the date of the run session and we return the run session data for that date
    // if the date is in the future that's an error, if the date is in the past we return the run session data for that date
    // date can't be empty, if it's empty that's an error
    const Message: &str = "Hello, world!";
    let json_response : serde_json::Value = serde_json::json!({ "message": Message });
    (StatusCode::OK, Json(json_response));

}

pub async fn create_runsession_handler() -> impl IntoResponse {
    const Message: &str = "Hello, world!";
    let json_response : serde_json::Value = serde_json::json!({ "message": Message });
    (StatusCode::OK, Json(json_response));
}