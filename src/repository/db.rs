use std::env::var;

use mongodb::{
  bson::{doc, to_document},
  error::{Error, Result},
  results::UpdateResult,
  Client, Collection,
};

use crate::model::{item::Item, table::Table};

#[derive(Debug)]
pub struct MongoDBRepository {
  client: Client,
  database_name: String,
}

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
  ) -> std::result::Result<Option<Table>, Error> {
    let filter = doc! { "table_id": &table_id };
    self.get_table_collection().find_one(filter, None).await
  }

  pub async fn add_item(
    &self,
    table_id: &str,
    item: &Item,
  ) -> Result<UpdateResult> {
    let filter = doc! { "table_id": &table_id };
    let bson_item = to_document(&item).unwrap();
    let update = doc! { "$addToSet": {"items": bson_item } };
    self
      .get_table_collection()
      .update_one(filter, update, None)
      .await
  }
}
