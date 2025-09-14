use axum::{routing::{get, post}, Router, extract::State};
use sqlx::PgPool;

mod handlers;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/ev-request", post(handlers::ev_request))
        .route("/ev-requests", get(handlers::ev_requests))
        .with_state(pool)
}