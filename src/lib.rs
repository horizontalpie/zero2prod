use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn greet() -> impl Responder {
    "Hello World"
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("{}, {}", form.name, form.email);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/hello_world", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
