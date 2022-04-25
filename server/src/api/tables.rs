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

  let res = mongodb_repository.get_table(&table_id).await;

  return match res {
    Ok(data) => match data {
      Some(data) => HttpResponse::Ok().json(data),
      None => HttpResponse::NotFound().body("Not found"),
    },
    Err(err) => {
      eprintln!("Error: {:?}", err);
      HttpResponse::InternalServerError()
        .body("Error: `get_table` operation failed.")
    }
  };
}

pub async fn add_table(
  table_factory: Json<TableFactory>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let table = Table {
    table_id: nanoid!(),
    items: table_factory.into_inner().items,
    created_at: Utc::now().to_string(),
  };
  let res = mongodb_repository.add_table(table).await;

  return match res {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("{:?}", err);
      HttpResponse::InternalServerError().body("add_table failed")
    }
  };
}

pub async fn delete_table(
  table_identifier: Path<TableIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let TableIdentifier { table_id } = table_identifier.into_inner();

  let res = mongodb_repository.delete_table(&table_id).await;

  match res {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => {
      eprintln!("{}", err);
      HttpResponse::InternalServerError().body("delete_table failed")
    }
  }
}
