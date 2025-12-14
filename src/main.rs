mod db;
mod handler;
mod models;

use axum::{response::Redirect, routing::get, Router};
use dotenvy::dotenv;
const BASE_PREFIX: &str = "/axum-crud";
use handler::{create_user, get_all_user};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::connect().await;

    let app = Router::new()
        .route(&format!("{}/", BASE_PREFIX), get(hello_world))
        .route(
            &format!("{}/users", BASE_PREFIX),
            get(get_all_user).post(create_user).with_state(pool),
        )
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
