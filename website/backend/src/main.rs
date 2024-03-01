mod api;
mod models;
mod repository;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use api::user_api::{create_user, get_user};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello, you should KILL YOURSELF NOW!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init();         // These might have to change
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
        .service(get_all_users)
        .service(hello)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

// use POST for destructive actions because you can't hit a POST action in the address bar of your browser
// Use GET when it's safe to allow a person to call an action. So a URL Should bring you to a confirmation page, rather than simply deleting the item
//POST is also more secure than GET, because you aren't sticking information into a URL
//One final note: POST can transmit a larger amount of information than GET. 'POST' has no size restrictions for transmitted data, whilst 'GET' is limited to 2048 characters.