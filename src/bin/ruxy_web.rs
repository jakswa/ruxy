use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = ruxy::settings::build().unwrap();
    let address = format!("0.0.0.0:{}", settings.web_port);
    let listener = TcpListener::bind(address).expect("failed to bind to port");
    ruxy::server::build_ssl(listener)?.await
}
