use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = ruxy::settings::build().unwrap();
    let address = format!("0.0.0.0:{}", settings.web_port);
    let listener = TcpListener::bind(address).expect("failed to bind to port");
    let db_pool = ruxy::settings::migrated_pool(&settings).await.unwrap();
    ruxy::server::build_ssl(listener, db_pool)?.await
}
