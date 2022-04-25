#[cfg(test)]
mod api_meta_tests {
  use actix_web::test::{call_and_read_body_json, call_service, read_body};
  use actix_web::{
    test,
    test::{call_and_read_body, init_service, TestRequest},
    web::{get, post, route, Bytes, Json},
    App,
  };
  use serde_json::{from_slice, json, Error};

  use crate::model::item::Item;
  use crate::model::table::Table;
  use crate::test_utils::get_mongodb_repository_data;
  use crate::{add_table, get_table};

  #[actix_web::test]
  async fn test_tables() {
    let app = init_service(
      App::new()
        .app_data(get_mongodb_repository_data())
        .route("/tables", post().to(add_table))
        .route("/tables/{table_id}", get().to(get_table)),
    )
    .await;

    let table = json!({"items": [{
      "item_id": "test_item_id",
      "table_id": "",
      "name": "test_item_name",
      "time": 10,
      "created_at": ""
    }]});
    let add_req = TestRequest::post()
      .uri("/tables")
      .set_json(table)
      .to_request();
    let add_res = call_service(&app, add_req).await;

    // Check if the update operation runs
    assert_eq!(
      add_res.status(),
      200,
      "Table should be added and response should be 200."
    );

    let body = read_body(add_res).await;
    let try_adding: Result<Table, Error> = from_slice(&body);

    // Check if response body is valid
    assert!(try_adding.is_ok(), "Response could not be parsed.");

    let get_req = TestRequest::get().uri("/tables").to_request();
    let tables: Vec<Table> = call_and_read_body_json(&app, get_req).await;
    let target_tables = tables.iter().find(|table| {
      table
        .items
        .iter()
        .find(|item| item.item_id == "test_item_id")
        .is_some()
    });

    // Check if inserted document is found in database
    assert!(target_tables.is_some(), "Table not found.");

    let target_table_id = match target_tables {
      Some(table) => table.table_id.clone(),
      None => panic!("Could not find document in database."),
    };

    let get_req = TestRequest::get()
      .uri(&format!("{}{}", "/tables/", target_table_id))
      .to_request();
    let get_res = call_service(&app, get_req).await;

    assert_eq!(
      get_res.status(),
      200,
      "Table should be retrieved with response 200"
    );

    let get_req = TestRequest::get()
      .uri(&format!("{}{}", "/tables/", target_table_id))
      .to_request();
    let item: Item = call_and_read_body_json(&app, get_req).await;

    assert_eq!(item.item_id, "test_item_id");
  }
}
