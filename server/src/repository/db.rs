use std::{env::var, result::Result as StdResult};

use mongodb::{
  bson::{doc, oid::ObjectId, to_document, Document},
  error::{Error, Result},
  results::{InsertOneResult, UpdateResult},
  Client, Collection, Cursor,
};

use crate::model::{item::Item, table::Table};

#[derive(Debug)]
pub struct MongoDBRepository {
  client: Client,
  database_name: String,
}

// Database access layer: Handles the direct CRUD functions against the database
//   driver and exposes a driver agnostic API against the controllers
impl MongoDBRepository {
  pub async fn init(database_name: String) -> MongoDBRepository {
    let mongo_uri = var("MONGO_URI")
      .expect("Error: failed to get MONGO_URI environment variable.");
    let client = Client::with_uri_str(&mongo_uri)
      .await
      .expect("Error: failed to connect to MongoDB instance.");
    MongoDBRepository {
      client,
      database_name,
    }
  }

  pub fn get_table_collection(&self) -> Collection<Table> {
    self
      .client
      .database(&self.database_name)
      .collection::<Table>("tables")
  }

  pub async fn get_table(
    &self,
    table_id: &str,
  ) -> StdResult<Option<Table>, Error> {
    let filter = doc! { "table_id": table_id };
    self.get_table_collection().find_one(filter, None).await
  }

  pub async fn add_table(
    &self,
    table_data: Table,
  ) -> StdResult<InsertOneResult, Error> {
    self
      .get_table_collection()
      .insert_one(table_data, None)
      .await
  }

  pub async fn delete_table(
    &self,
    table_id: &str,
  ) -> StdResult<Option<Table>, Error> {
    let filter = doc! { "table_id": table_id };
    self
      .get_table_collection()
      .find_one_and_delete(filter, None)
      .await
  }

  pub async fn add_item(&self, item: &Item) -> Result<UpdateResult> {
    let filter = doc! { "table_id": &item.table_id };
    let bson_item = to_document(&item).unwrap();
    let update = doc! { "$addToSet": {"items": bson_item } };
    self
      .get_table_collection()
      .update_one(filter, update, None)
      .await
  }

  pub async fn delete_item(
    &self,
    table_id: &str,
    item_id: &str,
  ) -> Result<UpdateResult> {
    let filter = doc! { "table_id": &table_id };
    let update = doc! { "$pull": {
      "items.$[].item_id": &item_id
    } };
    self
      .get_table_collection()
      .update_one(filter, update, None)
      .await
  }
}
