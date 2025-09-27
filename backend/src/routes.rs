use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use crate::{handlers, AppState};
use tower_http::cors::{Any, CorsLayer};

pub fn router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        // This line was missing and is needed to allow headers like Content-Type
        .allow_headers(Any);

    Router::new()
        .route("/", get(|| async { "EV Charging API is running" }))
        .route("/api/bookings", post(handlers::create_booking))
        .route("/api/bookings", get(handlers::get_all_bookings))
        .route("/api/bookings/:slot_id", get(handlers::get_booking))
        .route("/api/bookings/ev/:ev_id", get(handlers::get_bookings_by_ev))
        .route("/api/bookings/:slot_id/status/:status", put(handlers::update_booking_status))
        .with_state(state)
        .layer(cors)
}
