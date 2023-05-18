use std::io::Error;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use actix_cors::Cors;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
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
        })
        .listen(listener)?
        .run();

    Ok(server)
}
