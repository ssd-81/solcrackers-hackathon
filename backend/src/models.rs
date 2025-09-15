use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use chrono::{Utc, DateTime};


#[derive(Debug,Clone)]
pub struct EVpayload{
  pub ev_id:String,
  pub arrival_time:String,
  pub duration:u16,
  pub power_needed:i16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVresponse {
    pub status: String,
    pub slot_id: String,
    pub message: String,
}
//SlotBooking will be turend to schema for sql 
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SlotBooking {
    pub id: Option<i32>,
    pub slot_id: String,
    pub ev_id: String,
    pub arrival_time: DateTime<Utc>,
    pub duration: i32,
    pub power_needed: i64,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// For database insertions
#[derive(Debug, Clone)]
pub struct NewSlotBooking {
    pub slot_id: String,
    pub ev_id: String,
    pub arrival_time: DateTime<Utc>,
    pub duration: i32,
    pub power_needed: i64,
}

// For database model of responses
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EVResponseDB {
    pub id: Option<i32>,
    pub slot_id: String,
    pub status: String,
    pub message: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}