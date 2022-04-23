use actix_web::{
  web::{Data, Path},
  HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::repository::db::MongoDBRepository;

#[derive(Deserialize, Serialize)]
pub struct ItemIdentifier {
  table_id: String,
  item_id: String,
}

pub async fn get_item(
  item_identifier: Path<ItemIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  // extract params
  let table_id = &item_identifier.table_id;
  let item_id = &item_identifier.item_id;

  let res = mongodb_repository.get_table(&table_id).await;

  return match res {
    Ok(data) => match data {
      Some(data) => HttpResponse::Ok().json(data.items),
      None => HttpResponse::NotFound().body(format!(
        "Error: Item [item_id: {}, table_id:{}] not found",
        item_id, table_id
      )),
    },
    Err(_) => HttpResponse::InternalServerError().body("get_item failed"),
  };
}
