use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::model::item::Item;
use crate::utils::serialize_object_id;

#[derive(Deserialize, Serialize, Debug)]
pub struct Table {
  #[serde(
    rename = "_id",
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id"
  )]
  pub id: Option<ObjectId>,
  pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableQuery {
  #[serde(default)]
  pub keyword: String,
}
