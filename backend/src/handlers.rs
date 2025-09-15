// src/handlers.rs
use axum::Json;
use serde_json::json;

pub async fn ev_request() -> Json<serde_json::Value> {
    Json(json!({"result": "ev-request received"}))
}

pub async fn ev_requests() -> Json<serde_json::Value> {
    Json(json!({"result": "list of ev-requests"}))
}

pub async fn energy_status() -> Json<serde_json::Value> {
    Json(json!({"solar": "ok", "wind": "ok", "battery": "unknown", "grid": "ok"}))
}

pub async fn charging_plan() -> Json<serde_json::Value> {
    Json(json!({"plan": "dummy schedule"}))
}

pub async fn analytics() -> Json<serde_json::Value> {
    Json(json!({"savings": 0.0, "co2_avoided": 0.0}))
}
