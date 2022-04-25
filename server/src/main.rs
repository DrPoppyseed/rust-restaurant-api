use std::{
  env::{set_var, var},
  io::Result,
};

use actix_web::{
  middleware::Logger,
  web::{delete, get, post, route, Data},
  App, HttpServer,
};

use api::{
  items::{add_item, delete_item, get_item},
  meta::{echo, health_check, not_found},
  tables::{add_table, delete_table, get_table},
};
use repository::db::MongoDBRepository;

mod api;
mod model;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
  set_var("RUST_LOG", "actix_web=debug, actix_server=info");
  set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  let mongodb_repository: MongoDBRepository =
    MongoDBRepository::init("restaurant".to_string()).await;
  let mongodb_data = Data::new(mongodb_repository);
  let server_addr = var("ADDR").expect("Error: Failed to get ADDR.");

  // Actix-web spins up the same number of HTTP workers as the number of logical
  // CPUs found on the machine running the server by default. Hence, multi-threading
  // comes out of the box.
  HttpServer::new(move || {
    let logger = Logger::default();

    // Use .route notation instead of .service notation and macros because I
    // preferred to have less "magic" at compile time. Additionally, I found it
    // easier to implement unit tests when writing handlers without macros.
    App::new()
      .wrap(logger)
      .app_data(mongodb_data.clone())
      .route("/", get().to(health_check))
      .route("/", post().to(echo))
      .route("/tables", post().to(add_table))
      .route("/tables/{table_id}", get().to(get_table))
      .route("/tables/{table_id}", delete().to(delete_table))
      .route("/tables/{table_id}/items", post().to(add_item))
      .route("/tables/{table_id}/items/{item_id}", get().to(get_item))
      .route(
        "/tables/{table_id}/items/{item_id}",
        delete().to(delete_item),
      )
      .default_service(route().to(not_found))
  })
  .bind(&server_addr)?
  .run()
  .await
}
