use actix_web::{
  test::{call_and_read_body, init_service, TestRequest},
  web::{get, post, route, Bytes, Data},
  App,
};
use serde_json::json;

use crate::{echo, health_check, not_found, MongoDBRepository};

// Integration tests for api controllers
#[actix_web::test]
async fn api_meta_handlers_integration_test() {
  let app = init_service(
    App::new()
      .route("/", get().to(health_check))
      .route("/", post().to(echo))
      .default_service(route().to(not_found)),
  )
  .await;

  // Test health_check()
  let req = TestRequest::get().uri("/").to_request();
  let res = call_and_read_body(&app, req).await;
  assert_eq!(res, Bytes::from_static(b"Up!"));

  // Test echo()
  let msg = "hello";
  let echo_body = json!({ "message": msg });
  let req = TestRequest::post()
    .uri("/")
    .set_json(echo_body)
    .to_request();
  let res = call_and_read_body(&app, req).await;
  assert_eq!(res, Bytes::from_static(msg.as_ref()));

  // Test not_found()
  todo!();
}

#[actix_web::test]
async fn api_tables_integration_test() {
  let mongodb_repository =
    MongoDBRepository::init("restaurant".to_string()).await;
  let mongodb_data = Data::new(mongodb_repository);

  let app = init_service(App::new().app_data(mongodb_data)).await;

  // Test add_table()
  todo!();

  // Test get_table()
  todo!();
}
