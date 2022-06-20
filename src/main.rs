use actix_web::{App, HttpServer};
use controllers::posts::*;

pub mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(get_all_posts)
            .service(create_new_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}