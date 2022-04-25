#[cfg(test)]
mod test {
  use actix_web::test::{call_and_read_body_json, call_service, read_body};
  use actix_web::web::delete;
  use actix_web::{
    test::{init_service, TestRequest},
    web::{get, post, route, Bytes, Json},
    App,
  };
  use serde_json::{json, Error};

  use crate::model::item::Item;
  use crate::model::table::Table;
  use crate::test_utils::get_mongodb_repository_data;
  use crate::{add_item, add_table, delete_item, get_item};

  #[actix_web::test]
  async fn test_items() {
    let app = init_service(
      App::new()
        .app_data(get_mongodb_repository_data())
        .route("/tables", post().to(add_table))
        .route("/tables/{table_id}/items", post().to(add_item))
        .route("/tables/{table_id}/items/{item_id}", get().to(get_item))
        .route(
          "/tables/{table_id}/items/{item_id}",
          delete().to(delete_item),
        ),
    )
    .await;

    // Setup the table in order to test the item handlers
    let table = json!({"items": [{
      "item_id": "test_item_id_2",
      "table_id": "",
      "name": "test_item_name_2",
      "time": 10,
      "created_at": ""
    }]});
    TestRequest::post()
      .uri("/tables")
      .set_json(table)
      .to_request();

    let get_req = TestRequest::get().uri("/tables").to_request();
    let tables: Vec<Table> = call_and_read_body_json(&app, get_req).await;
    let target_table = match tables.iter().find(|table| {
      table
        .items
        .iter()
        .find(|item| item.item_id == "test_item_id")
        .is_some()
    }) {
      Some(table) => table.clone(),
      None => panic!("Could not find table in collection."),
    };
    let Table {
      table_id, items, ..
    } = target_table;

    let target_item =
      match items.iter().find(|item| item.item_id == "test_item_id_2") {
        Some(item) => item,
        None => panic!("Could not find item in table."),
      };
    let Item { item_id, .. } = target_item;

    // Test get_item()
    let get_req = TestRequest::get()
      .uri(&format!(
        "{}{}{}{}",
        "/tables/", table_id, "/items/", item_id
      ))
      .to_request();
    let get_res = call_service(&app, get_req).await;

    assert_eq!(
      get_res.status(),
      200,
      "Item should be retrieved with response 200."
    );

    // Test add_item()
    let item = json!({
      "item_id": "test_item_id_3",
      "table_id": "",
      "name": "test_item_name_3",
      "time": 15,
      "created_at": ""
    });
    let add_req = TestRequest::post()
      .uri(&format!("{}{}{}", "/tables", table_id, "/items"))
      .set_json(item)
      .to_request();
    let add_res = call_service(&app, add_req).await;

    assert_eq!(
      add_res.status(),
      200,
      "Item should be added with response 200"
    );

    // Test delete_item()
    let del_req = TestRequest::delete()
      .uri(&format!(
        "{}{}{}{}",
        "/tables/", table_id, "/items/", item_id
      ))
      .to_request();
    let del_res = call_service(&app, del_req).await;

    assert_eq!(
      del_res.status(),
      200,
      "Item should be deleted with response 200."
    );
  }
}
