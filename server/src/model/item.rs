use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
  pub item_id: String,
  pub table_id: String,
  /// name: The name of the item
  pub name: String,
  /// time: The amount of time required to make the item
  pub time: u32,
  /// created_at: The time at which the item was ordered
  pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct ItemIdentifier {
  pub table_id: String,
  pub item_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ItemFactory {
  pub name: String,
  pub time: u32,
}
