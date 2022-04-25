use actix_web::web::Data;

use crate::MongoDBRepository;

pub async fn get_mongodb_repository_data() -> Data<MongoDBRepository> {
  let mongodb_repository =
    MongoDBRepository::init("restaurant".to_string()).await;
  Data::new(mongodb_repository)
}
