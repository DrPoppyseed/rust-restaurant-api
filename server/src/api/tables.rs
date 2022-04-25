use actix_web::{
  web::{Data, Json, Path},
  HttpResponse,
};
use chrono::Utc;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::model::{
  item::Item,
  table::{Table, TableFactory, TableIdentifier},
};
use crate::repository::db::MongoDBRepository;

pub async fn get_table(
  table_identifier: Path<TableIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let TableIdentifier { table_id } = table_identifier.into_inner();

  mongodb_repository
    .get_table(&table_id)
    .await
    .map(|data| match data {
      Some(data) => HttpResponse::Ok().json(data),
      None => HttpResponse::NotFound().body("Not found"),
    })
    .map_err(|_| {
      HttpResponse::InternalServerError()
        .body("Error: `get_table` operation failed.")
    })
    .unwrap()
}

fn table_creator(table_factory: TableFactory) -> Table {
  Table {
    table_id: nanoid!(),
    items: table_factory.items,
    created_at: Utc::now().to_string(),
  }
}

pub async fn add_table(
  table_factory: Json<TableFactory>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  mongodb_repository
    .add_table(table_creator(table_factory.into_inner()))
    .await
    .map(|_| HttpResponse::Ok().finish())
    .map_err(|_| HttpResponse::InternalServerError().body("add_table failed"))
    .unwrap()
}

pub async fn delete_table(
  table_identifier: Path<TableIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let TableIdentifier { table_id } = table_identifier.into_inner();

  mongodb_repository
    .delete_table(&table_id)
    .await
    .map(|_| HttpResponse::Ok().finish())
    .map_err(|_| {
      HttpResponse::InternalServerError().body("delete_table failed")
    })
    .unwrap()
}
