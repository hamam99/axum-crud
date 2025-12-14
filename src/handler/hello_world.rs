use axum::Json;

pub async fn hello_world() -> Json<&'static str> {
    Json("Hello, world!")
}
