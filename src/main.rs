pub mod db;
pub mod handler;
pub mod models;
pub mod routes;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = match env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_) => "0.0.0.0:3000".to_string(),
    };

    let app = routes::app_route::create_router().await;
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
