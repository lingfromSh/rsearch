use actix_web::{App, HttpServer};

mod apis;
mod common;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(apis::search::search)
    })
        .server_hostname("rsearch")
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}