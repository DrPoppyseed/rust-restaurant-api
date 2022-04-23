use mongodb::bson::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
  id: String,
  name: String,
  created_at: Timestamp,
  time: u32
}
