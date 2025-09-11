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
