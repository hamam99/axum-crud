use axum::{extract::State, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUser, User};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(payload.username)
    .bind(payload.email)
    .fetch_one(&pool)
    .await
    .expect("Create user failed");

    return Json(user);
}
