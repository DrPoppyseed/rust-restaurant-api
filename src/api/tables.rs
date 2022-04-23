use actix_web::{
  web::{Data, Path},
  HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::repository::db::MongoDBRepository;

#[derive(Deserialize, Serialize)]
pub struct TableIdentifier {
  table_id: String,
}

pub async fn get_table(
  table_identifier: Path<TableIdentifier>,
  mongodb_repository: Data<MongoDBRepository>,
) -> HttpResponse {
  let res = mongodb_repository
    .get_table(&table_identifier.into_inner().table_id)
    .await;

  return match res {
    Ok(data) => match data {
      Some(data) => HttpResponse::Ok().json(data),
      None => HttpResponse::NotFound().body("Not found"),
    },
    Err(err) => {
      eprintln!("{:?}", err);
      HttpResponse::InternalServerError().body("get_table failed")
    }
  };
}
// #[post("/tables/{table_id}")]
// pub async fn add_item(table_identifier)
