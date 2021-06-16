use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod employees;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate listenfd;

mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(employees::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}