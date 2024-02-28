mod api;
mod models;
mod repository;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello, you should KILL YOURSELF NOW!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new().service(hello)})
        .bind("0.0.0.0:8080")?
        .run()
        .await
}