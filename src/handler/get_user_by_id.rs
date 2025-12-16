// pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<User> {
//     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
//         .fetch_one(&pool)
//         .await
//         .expect("Get user by id failed");

//     return Json(user);
// }

use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("User not found");

    return Json(user);
}
