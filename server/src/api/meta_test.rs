#[cfg(test)]
mod api_meta_tests {
  use crate::{echo, health_check, not_found};
  use actix_web::{
    test::{call_and_read_body, init_service, TestRequest},
    web::{get, post, route, Bytes, Json},
    App,
  };
  use serde_json::json;

  #[actix_web::test]
  async fn test_health_check() {
    let app = init_service(App::new().route("/", get().to(health_check))).await;

    let req = TestRequest::get().uri("/").to_request();
    let res = call_and_read_body(&app, req).await;
    assert_eq!(res, Bytes::from_static(b"Up!"));
  }

  #[actix_web::test]
  async fn test_not_found() {
    let app =
      init_service(App::new().default_service(route().to(not_found))).await;

    let req = TestRequest::get().uri("/ddd").to_request();
    let res = call_and_read_body(&app, req).await;
    assert_eq!(res, Bytes::from_static(b"404 page not found"))
  }

  #[actix_web::test]
  async fn test_echo() {
    let app = init_service(App::new().route("/", post().to(echo))).await;

    let msg = "hello";
    let echo_body = json!({ "message": msg });
    let req = TestRequest::post()
      .uri("/")
      .set_json(echo_body)
      .to_request();
    let res = call_and_read_body(&app, req).await;
    assert_eq!(res, Bytes::from_static(msg.as_ref()));
  }
}
