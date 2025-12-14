use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn connect() -> Pool<Postgres> {
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "".to_string(),
    };

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Database connection failed")
}
