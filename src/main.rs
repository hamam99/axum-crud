use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
async fn hello_world() -> &'static str {
    return "Hello, world!";
}
