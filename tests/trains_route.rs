use actix_web::client::Client;
use std::net::TcpListener;

#[actix_rt::test]
async fn trains_route_works() {
    let base_uri = spawn_server();
    let uri = format!("{}/trains", base_uri);
    let resp = Client::default().get(uri).send().await;
}

fn spawn_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = ruxy::server::build_ssl(listener).expect("failed to bind");
    tokio::spawn(server);
    format!("https://127.0.0.1:{}", port)
}
