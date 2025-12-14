use crate::handler::{
    create_user::create_user, get_all_user::get_all_user, not_found_handler::not_found_handler,
};
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

const BASE_PREFIX: &str = "/axum-crud";

pub fn create_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            &format!("{}/", BASE_PREFIX),
            get(get_all_user).with_state(pool.clone()),
        )
        .route(
            &format!("{}/users", BASE_PREFIX),
            get(get_all_user).post(create_user).with_state(pool),
        )
        .fallback(not_found_handler)
}
