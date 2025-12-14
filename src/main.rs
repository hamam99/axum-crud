mod db;
mod handler;
mod models;

use std::env;

use axum::{response::Redirect, routing::get, Json, Router};
use dotenvy::dotenv;
const BASE_PREFIX: &str = "/axum-crud";
use handler::{create_user, get_all_user};

use crate::handler::not_found_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = match env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_) => "0.0.0.0:3000".to_string(),
    };

    let pool = db::connect().await;

    let app = Router::new()
        .route(&format!("{}/", BASE_PREFIX), get(hello_world))
        .route(
            &format!("{}/users", BASE_PREFIX),
            get(get_all_user).post(create_user).with_state(pool),
        )
        .fallback(not_found_handler);
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
async fn hello_world() -> Json<&'static str> {
    return Json("Hello, world!");
}

async fn handler_redirect() -> Redirect {
    Redirect::to(&format!("{}/", BASE_PREFIX))
}
