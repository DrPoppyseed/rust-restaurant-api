use actix_web::{
  web::{Data, Json, Path},
  HttpResponse,
};
use chrono::Utc;
use nanoid::nanoid;
use serde_json::from_str;

use crate::model::{
  item::{Item, ItemFactory, ItemIdentifier},
  table::TableIdentifier,
};
use crate::repository::db::MongoDBRepository;

// Handle item not found responses
fn item_not_found(table_id: &str, item_id: &str) -> HttpResponse {
  HttpResponse::NotFound().body(format!(
    "Error: Item [item_id: {}, table_id:{}] not found",
    &item_id, &table_id
  ))
}

pub async fn get_item(
  item_identifier: Path<ItemIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let ItemIdentifier { table_id, item_id } = item_identifier.into_inner();

  mongodb_repository
    .get_table(&table_id)
    .await
    .map(|data| match data {
      Some(table) => {
        let item = table.items.iter().find(|item| item.item_id.eq(&item_id));
        match item {
          Some(item) => HttpResponse::Ok().json(item),
          None => item_not_found(&table_id, &item_id),
        }
      }
      None => item_not_found(&table_id, &item_id),
    })
    .map_err(|_| HttpResponse::InternalServerError().body("get_item failed"))
    .unwrap()
}

fn item_creator(
  table_identifier: TableIdentifier,
  item_factory: ItemFactory,
) -> Item {
  let TableIdentifier { table_id } = table_identifier;
  let ItemFactory { name, time } = item_factory;

  Item {
    item_id: nanoid!(),
    table_id: table_id,
    name: name,
    time: time,
    created_at: Utc::now().to_string(),
  }
}

pub async fn add_item(
  table_identifier: Path<TableIdentifier>,
  item_factory: Json<ItemFactory>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  mongodb_repository
    .add_item(&item_creator(
      table_identifier.into_inner(),
      item_factory.into_inner(),
    ))
    .await
    .map(|data| HttpResponse::Ok().json(data))
    .map_err(|_| HttpResponse::InternalServerError().body("add_item failed"))
    .unwrap()
}

pub async fn delete_item(
  item_identifier: Path<ItemIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let ItemIdentifier { table_id, item_id } = item_identifier.into_inner();

  mongodb_repository
    .delete_item(&table_id, &item_id)
    .await
    .map(|_| HttpResponse::Ok().finish())
    .map_err(|_| HttpResponse::InternalServerError().body("delete_item failed"))
    .unwrap()
}
