use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUser, User};

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING *",
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(id)
    .fetch_one(&pool)
    .await
    .expect("Update user failed");

    return Json(user);
}
