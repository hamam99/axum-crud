use axum::Json;

use crate::models::response::Response;

pub async fn not_found_handler() -> Json<Response> {
    let res = Response {
        message: "URL not found".to_string(),
    };

    Json(res)
}
