use serde::{Deserialize, Serialize};

use crate::model::item::Item;

#[derive(Deserialize, Serialize, Debug)]
pub struct Table {
  /// id is given a uuid and represents the business id of the table
  pub table_id: String,
  /// the menu items that have been ordered for the table
  pub items: Vec<Item>,
  /// the time the table item was created at
  pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct TableIdentifier {
  pub table_id: String,
}

/// The TableFactory struct is used as the Json request body for post requests
/// trying to add new tables to the database.
#[derive(Deserialize, Serialize)]
pub struct TableFactory {
  pub items: Vec<Item>,
}
