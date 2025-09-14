use axum::{extract::State, Json, response::Json as ResponseJson};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct EvRequestPayload {
    pub user_id: i32,
    pub slot_start: NaiveDateTime,
    pub slot_end: NaiveDateTime,
}

pub async fn ev_request(
    State(pool): State<PgPool>,
    Json(payload): Json<EvRequestPayload>,
) -> Result<ResponseJson<Value>, String> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO ev_requests (user_id, slot_start, slot_end)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.user_id,
        payload.slot_start,
        payload.slot_end
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(ResponseJson(json!({"id": rec.id, "message": "Created successfully"})))
}

pub async fn ev_requests(
    State(pool): State<PgPool>,
) -> Result<ResponseJson<Value>, String> {
    let recs = sqlx::query!(
        "SELECT id, user_id, slot_start, slot_end, created_at FROM ev_requests"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    let result: Vec<Value> = recs
        .iter()
        .map(|r| json!({
            "id": r.id,
            "user_id": r.user_id,
            "slot_start": r.slot_start.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            "slot_end": r.slot_end.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            "created_at": r.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        }))
        .collect();

    Ok(ResponseJson(json!(result)))
}