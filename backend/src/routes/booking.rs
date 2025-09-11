use chrono::{Utc, DateTime};
#[derive(Debug,Clone)]
pub struct EVpayload{
  pub ev_id:String,
  pub arrival_time:String,
  pub duration:u16,
  pub power_needed:i16,
}
#[derive(Debug,Clone)]
pub struct EVresponse{
  pub status:String,
  pub slot_id:String,
  pub message:String,
}
//SlotBooking will be turend to schema for sql 
#[derive(Debug,Clone)]
pub struct SlotBooking{
  pub slot_id:String,
  pub ev_id:String,
  pub arrival_time:DateTime<Utc>,
  pub duration:u32,
  pub power_needed:u64,
}
