use actix_web::client::Client;
use actix_web::{get, App, Error, HttpResponse, HttpServer};

#[get("/trains")]
async fn trains() -> Result<HttpResponse, Error> {
    let trains_url = format!(
        "http://developer.itsmarta.com/RealtimeTrain/RestServiceNextTrain/GetRealtimeArrivals?apikey={}",
        std::env::var("MARTA_TRAIN_API_KEY").expect("NEED API KEY")
    );
    let mut resp = Client::default()
        .get(trains_url)
        .send()
        .await
        .map_err(Error::from)?;
    Ok(HttpResponse::build(resp.status()).body(resp.body().await?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(trains))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
