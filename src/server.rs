use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::net::TcpListener;

use crate::http_caches::*;
use crate::routes::*;

pub fn build_ssl(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let http_caches = Data::new(HttpCaches::default());

    let server = HttpServer::new(move || {
        App::new()
            .service(get_trains)
            .service(get_buses)
            .wrap(actix_cors::Cors::permissive()) // public API
            .wrap(actix_web::middleware::Logger::default())
            .app_data(http_caches.clone())
    })
    .workers(2)
    .listen_openssl(listener, ssl_builder())?
    .run();
    // .run();
    Ok(server)
}

// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
fn ssl_builder() -> openssl::ssl::SslAcceptorBuilder {
    let settings = crate::settings::build().unwrap();
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file(settings.ssl_key, SslFiletype::PEM)
        .unwrap();
    ssl_builder
        .set_certificate_chain_file(settings.ssl_cert)
        .unwrap();
    ssl_builder
}
