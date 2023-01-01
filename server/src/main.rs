use actix_web::{App, HttpServer};

mod apis;
mod common;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(apis::search::search)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}