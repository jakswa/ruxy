use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use ruxy::http_caches::*;
use ruxy::routes::*;
use ruxy::settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let http_caches = Data::new(HttpCaches::default());
    let settings = settings::build().unwrap();

    HttpServer::new(move || {
        App::new()
            .service(get_trains)
            .service(get_buses)
            .wrap(actix_cors::Cors::permissive()) // public API
            .wrap(actix_web::middleware::Logger::default())
            .app_data(http_caches.clone())
    })
    .workers(10)
    .bind_openssl(
        format!("0.0.0.0:{}", settings.web_port),
        ssl_builder(settings.ssl_key, settings.ssl_cert),
    )?
    .run()
    .await
}

// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
fn ssl_builder(ssl_key: String, ssl_cert: String) -> openssl::ssl::SslAcceptorBuilder {
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file(ssl_key, SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file(ssl_cert).unwrap();
    ssl_builder
}
