use axum::Json;

use crate::models::{create_user::CreateUser, response::Response, user::User};

pub async fn not_found_handler() -> Json<Response> {
    let res = Response {
        message: "URL not found".to_string(),
    };

    Json(res)
}
