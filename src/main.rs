use std::{env, io::Result};

use actix_web::{
  middleware::Logger,
  web::{get, Data},
  App, HttpServer,
};

use api::{health_check, items::get_item, tables::get_table};
use repository::db::MongoDBRepository;

mod api;
mod model;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
  env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  let mongodb_repository: MongoDBRepository =
    MongoDBRepository::init("restaurant".to_string()).await;
  let mongodb_data = Data::new(mongodb_repository);

  // Use .route notation instead of .service notation and macros because I
  // preferred to have less "magic" at compile time. Additionally, I found it
  // easier to implement unit tests when writing handlers without macros.
  HttpServer::new(move || {
    let logger = Logger::default();

    App::new()
      .wrap(logger)
      .app_data(mongodb_data.clone())
      .route("/", get().to(health_check))
      .route("/tables/{table_id}", get().to(get_table))
      .route("/tables/{table_id}/items/{item_id}", get().to(get_item))
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
