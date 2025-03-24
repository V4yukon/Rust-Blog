// Owner: Rust_Blog
// Contributors: Rust_Blog
// Reviewer: Rust_Blog
// Created: 04/05/2021
// Modified: 04/05/2021
// Version: 1.0
// Type: Source Code
// Description: Main file for the Rust Blog project 
use actix_web::{HttpRequest, Responder};


async fn greet(req: HttpRequest) -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}