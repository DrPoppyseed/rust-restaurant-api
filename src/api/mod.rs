use actix_web::{HttpResponse, Responder};

pub mod items;
pub mod tables;
#[cfg(test)]
mod test;

pub async fn health_check() -> impl Responder {
  println!("Server console is running.");
  HttpResponse::Ok().body("Up!")
}
