use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::{
    models::{EVpayload, EVresponse, NewSlotBooking, EVResponseDB},
    repository::SlotBookingRepository,
    AppState,
};

// Create a new slot booking
#[axum::debug_handler]
pub async fn create_booking(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EVpayload>,
) -> Json<EVresponse> {
    let repo = SlotBookingRepository::new(state.db_pool.clone());
    
    // Parse arrival time (assuming it's in ISO format)
    let arrival_time = match payload.arrival_time.parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        Err(_) => {
            let response = EVresponse {
                status: "error".to_string(),
                slot_id: "".to_string(),
                message: "Invalid arrival_time format. Use ISO 8601 format (e.g., 2023-12-01T10:00:00Z)".to_string(),
            };
            return Json(response);
        }
    };

    // Generate unique slot ID
    let slot_id = SlotBookingRepository::generate_slot_id();

    // Create new booking
    let new_booking = NewSlotBooking {
        slot_id: slot_id.clone(),
        ev_id: payload.ev_id.clone(),
        arrival_time,
        duration: payload.duration as i32,
        power_needed: payload.power_needed as i64,
    };

    match repo.create_slot_booking(new_booking).await {
        Ok(_booking) => {
            // Log the response
            let response_log = EVResponseDB {
                id: None,
                slot_id: slot_id.clone(),
                status: "success".to_string(),
                message: Some("Booking created successfully".to_string()),
                created_at: None,
            };
            let _ = repo.create_ev_response(&response_log).await;

            let response = EVresponse {
                status: "success".to_string(),
                slot_id,
                message: "Booking created successfully".to_string(),
            };
            Json(response)
        }
        Err(e) => {
            println!("Error creating booking: {:?}", e);
            let response = EVresponse {
                status: "error".to_string(),
                slot_id: "".to_string(),
                message: format!("Failed to create booking: {}", e),
            };
            Json(response)
        }
    }
}

// Get booking by slot ID
#[axum::debug_handler]
pub async fn get_booking(
    State(state): State<Arc<AppState>>,
    Path(slot_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let repo = SlotBookingRepository::new(state.db_pool.clone());

    match repo.get_slot_booking_by_id(&slot_id).await {
        Ok(Some(booking)) => Ok(Json(serde_json::json!({
            "status": "success",
            "data": booking
        }))),
        Ok(None) => Ok(Json(serde_json::json!({
            "status": "error",
            "message": "Booking not found"
        }))),
        Err(e) => {
            println!("Error fetching booking: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get all bookings
#[axum::debug_handler]
pub async fn get_all_bookings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let repo = SlotBookingRepository::new(state.db_pool.clone());

    match repo.get_all_slot_bookings().await {
        Ok(bookings) => Ok(Json(serde_json::json!({
            "status": "success",
            "data": bookings
        }))),
        Err(e) => {
            println!("Error fetching bookings: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get bookings by EV ID
#[axum::debug_handler]
pub async fn get_bookings_by_ev(
    State(state): State<Arc<AppState>>,
    Path(ev_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let repo = SlotBookingRepository::new(state.db_pool.clone());

    match repo.get_bookings_by_ev_id(&ev_id).await {
        Ok(bookings) => Ok(Json(serde_json::json!({
            "status": "success",
            "data": bookings
        }))),
        Err(e) => {
            println!("Error fetching bookings: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Update booking status
#[axum::debug_handler]
pub async fn update_booking_status(
    State(state): State<Arc<AppState>>,
    Path((slot_id, status)): Path<(String, String)>,
) -> Json<EVresponse> {
    let repo = SlotBookingRepository::new(state.db_pool.clone());

    match repo.update_booking_status(&slot_id, &status).await {
        Ok(_) => {
            let response = EVresponse {
                status: "success".to_string(),
                slot_id: slot_id.clone(),
                message: format!("Status updated to: {}", status),
            };
            Json(response)
        }
        Err(e) => {
            println!("Error updating booking status: {:?}", e);
            let response = EVresponse {
                status: "error".to_string(),
                slot_id,
                message: format!("Failed to update status: {}", e),
            };
            Json(response)
        }
    }
}
