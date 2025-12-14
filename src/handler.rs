use crate::models::{CreateUser, Response, User};
use axum::{extract::State, Json};
use sqlx::PgPool;
use uuid::Uuid;

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

pub async fn get_all_user(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("Fetch users failed");

    Json(users)
}

pub async fn not_found_handler() -> Json<Response> {
    let res = Response {
        message: "URL not found".to_string(),
    };

    Json(res)
}
