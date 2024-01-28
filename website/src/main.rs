use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse};

fn serve_html(file_name: &str) -> HttpResponse {
    let file_path = format!("static/{}", file_name);

    match std::fs::read_to_string(&file_path) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content),
        Err(_) => HttpResponse::NotFound().body("Page not found"),
    }
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            // Serve the "static" directory
            .service(fs::Files::new("/", "static").index_file("index.html"))
            // Define routes for each HTML file
            .route("/page1", web::get().to("page1")) // Burde endres til || serve_html("filepath.html")
            .route("/page2", web::get().to("page2"))
            .route("/page3", web::get().to("page3"))
            .route("/page4", web::get().to("page4"))
            // Define other API routes
            .route("/api", web::get().to("index.html"))
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
    .await
    .expect("Failed to run the server");
}
