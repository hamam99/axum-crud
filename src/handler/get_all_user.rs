use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::user::User;

pub async fn get_all_user(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("Fetch users failed");

    Json(users)
}
