use axum::{response::Redirect, routing::get, Json, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
const BASE_PREFIX: &str = "/axum-test";

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env");
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "".to_string(),
    };
    println!("DATABASE_URL: {}", database_url);

    let app = Router::new()
        .route(&format!("{}/", BASE_PREFIX), get(hello_world))
        // .fallback(handler_redirect);
        .fallback("Not Found");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
async fn hello_world() -> &'static str {
    return "Hello, world!";
}

async fn handler_redirect() -> Redirect {
    Redirect::to(&format!("{}/", BASE_PREFIX))
}
