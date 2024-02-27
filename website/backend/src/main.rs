use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "text/plain")
        .body("Hello, Actix web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}