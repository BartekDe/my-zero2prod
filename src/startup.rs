use crate::routes::{health_check, subscribe};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::io::Error;
use std::net::TcpListener;
use sqlx::{PgPool};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
