use std::io::Error;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::{JsonBody, Server};
use actix_cors::Cors;
use std::net::TcpListener;
use actix_web::dev::JsonBody::Body;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| {
            let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(3600);

            App::new()
                .wrap(cors)
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
