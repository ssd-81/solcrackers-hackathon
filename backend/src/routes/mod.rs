pub mod analytics;
pub mod booking;
pub mod energy;
pub mod plan;

use axum::{routing::{get, post}, Router};

mod super_handlers {
    // re-export handlers from crate root handlers module
    pub use crate::handlers::{ev_request, ev_requests, energy_status, charging_plan, analytics};
}

pub fn router() -> Router {
    Router::new()
        .route("/ev-request", post(super_handlers::ev_request))
        .route("/ev-requests", get(super_handlers::ev_requests))
        .route("/energy-status", get(super_handlers::energy_status))
        .route("/charging-plan", get(super_handlers::charging_plan))
        .route("/analytics", get(super_handlers::analytics))
}
