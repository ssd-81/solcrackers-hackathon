use crate::models::{SlotBooking, NewSlotBooking, EVResponseDB};
use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;

pub struct SlotBookingRepository {
    pool: PgPool,
}

impl SlotBookingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_slot_booking(&self, booking: NewSlotBooking) -> Result<SlotBooking> {
        let row = sqlx::query_as!(
            SlotBooking,
            r#"
            INSERT INTO slot_bookings (slot_id, ev_id, arrival_time, duration, power_needed)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, slot_id, ev_id, arrival_time, duration, power_needed, status, created_at, updated_at
            "#,
            booking.slot_id,
            booking.ev_id,
            booking.arrival_time,
            booking.duration,
            booking.power_needed
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_slot_booking_by_id(&self, slot_id: &str) -> Result<Option<SlotBooking>> {
        let row = sqlx::query_as!(
            SlotBooking,
            "SELECT id, slot_id, ev_id, arrival_time, duration, power_needed, status, created_at, updated_at FROM slot_bookings WHERE slot_id = $1",
            slot_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn get_all_slot_bookings(&self) -> Result<Vec<SlotBooking>> {
        let rows = sqlx::query_as!(
            SlotBooking,
            "SELECT id, slot_id, ev_id, arrival_time, duration, power_needed, status, created_at, updated_at FROM slot_bookings ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn update_booking_status(&self, slot_id: &str, status: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE slot_bookings SET status = $1 WHERE slot_id = $2",
            status,
            slot_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_bookings_by_ev_id(&self, ev_id: &str) -> Result<Vec<SlotBooking>> {
        let rows = sqlx::query_as!(
            SlotBooking,
            "SELECT id, slot_id, ev_id, arrival_time, duration, power_needed, status, created_at, updated_at FROM slot_bookings WHERE ev_id = $1 ORDER BY arrival_time",
            ev_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn create_ev_response(&self, response: &EVResponseDB) -> Result<()> {
        sqlx::query!(
            "INSERT INTO ev_responses (slot_id, status, message) VALUES ($1, $2, $3)",
            response.slot_id,
            response.status,
            response.message
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub fn generate_slot_id() -> String {
        format!("SLOT_{}", Uuid::new_v4().to_string()[..8].to_uppercase())
    }
}
