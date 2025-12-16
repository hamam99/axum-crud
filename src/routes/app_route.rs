use crate::db::connect::connect;
use crate::handler::{
    create_user::create_user, get_all_user::get_all_user, get_user_by_id::get_user_by_id,
    not_found_handler::not_found_handler,
};
use axum::{routing::get, Router};

const BASE_PREFIX: &str = "/axum-crud";

pub async fn create_router() -> Router {
    let pool: sqlx::Pool<sqlx::Postgres> = connect().await;

    Router::new()
        .route(&format!("{}/", BASE_PREFIX), get(get_all_user))
        .with_state(pool.clone())
        .route(
            &format!("{}/users", BASE_PREFIX),
            get(get_all_user).post(create_user).with_state(pool.clone()),
        )
        .route(
            &format!("{}/users/{{id}}", BASE_PREFIX),
            get(get_user_by_id).with_state(pool.clone()),
        )
        .fallback(not_found_handler)
}
