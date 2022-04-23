use actix_web::test::{call_and_read_body, init_service, TestRequest};
use actix_web::web::{get, Bytes, Data};
use actix_web::App;

use crate::{health_check, MongoDBRepository};

// Integration tests for api controllers
#[actix_web::test]
async fn test_tables() {
  let mongodb_repository =
    MongoDBRepository::init("restaurant".to_string()).await;
  let mongodb_data = Data::new(mongodb_repository);

  let app = init_service(
    App::new()
      .app_data(mongodb_data.clone())
      .route("/", get().to(health_check)),
  )
  .await;

  // Test health_check()
  let req = TestRequest::get().uri("/").to_request();
  let res = call_and_read_body(&app, req).await;
  assert_eq!(res, Bytes::from_static(b"Up!"));
}
