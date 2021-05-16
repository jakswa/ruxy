use std::fs::File;
use std::io::BufReader;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use sqlx::sqlite::SqlitePool;
use std::net::TcpListener;

use crate::http_caches::*;
use crate::routes::*;

pub fn build_ssl(listener: TcpListener, pool: SqlitePool) -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let http_caches = Data::new(HttpCaches::default());
    let pool = Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(get_trains)
            .service(get_buses)
            //.wrap(actix_cors::Cors::permissive()) // public API. edit: broken with beta?
            .wrap(actix_web::middleware::Logger::default())
            .app_data(http_caches.clone())
            .app_data(pool.clone())
    })
    .workers(2)
    .listen_rustls(listener, ssl_builder())?
    .run();
    // .run();
    Ok(server)
}

// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
fn ssl_builder() -> ServerConfig {
    let settings = crate::settings::build().unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(settings.ssl_cert).unwrap());
    let key_file = &mut BufReader::new(File::open(settings.ssl_key).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}
