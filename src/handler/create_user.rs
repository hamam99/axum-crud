use axum::{extract::State, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{create_user::CreateUser, response::Response, user::User};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, email) VALUES ($1, $2, $3) RETURNING *",
        Uuid::new_v4(),
        payload.username,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Create user failed");

    Json(user)
}
