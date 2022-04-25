use actix_web::{web::Json, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoBody {
  pub message: String,
}

pub async fn health_check() -> impl Responder {
  println!("Server console is running.");
  HttpResponse::Ok().body("Up!")
}

pub async fn echo(echo_body: Json<EchoBody>) -> impl Responder {
  HttpResponse::Ok().body(format!("{}", echo_body.message))
}

pub async fn not_found() -> impl Responder {
  HttpResponse::Ok().body("404 page not found")
}
