use actix_web::client::Client;
use actix_web::web::{Bytes, Data};
use actix_web::{get, http::StatusCode, App, Error, HttpResponse, HttpServer};
use env_logger::Env;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug)]
struct SharedResp {
    expiry: Instant,
    ttl: Duration,
    url: String,
    body: Bytes,
    status: StatusCode,
}

struct HttpCaches {
    trains: RwLock<SharedResp>,
    buses: RwLock<SharedResp>,
}

impl HttpCaches {
    pub fn new() -> HttpCaches {
        HttpCaches {
            trains: RwLock::new(SharedResp::for_trains()),
            buses: RwLock::new(SharedResp::for_buses()),
        }
    }
}

impl SharedResp {
    pub fn for_trains() -> SharedResp {
        let train_url =format!(
            "http://developer.itsmarta.com/RealtimeTrain/RestServiceNextTrain/GetRealtimeArrivals?apikey={}",
            std::env::var("MARTA_TRAIN_API_KEY").expect("NEED API KEY")
        );
        SharedResp::new_blank(train_url)
    }

    pub fn for_buses() -> SharedResp {
        SharedResp::new_blank(
            "http://developer.itsmarta.com/BRDRestService/RestBusRealTimeService/GetAllBus"
                .to_string(),
        )
    }

    pub fn new_blank(url: String) -> SharedResp {
        SharedResp {
            url,
            expiry: Instant::now(),
            ttl: Duration::from_secs(10),
            body: Bytes::from(&b"<unused>"[..]),
            status: StatusCode::OK,
        }
    }

    pub fn unexpired(&self) -> bool {
        self.expiry > Instant::now()
    }

    pub async fn update(&mut self) -> Result<&SharedResp, Error> {
        self.expiry = Instant::now() + self.ttl;
        let resp = Client::default().get(&self.url).send().await;

        match resp {
            Ok(mut res) => {
                self.status = res.status();
                self.body = res.body().await?;
            }
            Err(err) => {
                self.body = Bytes::from(format!("received {}", err));
                self.status = match err {
                    actix_web::client::SendRequestError::Timeout => StatusCode::REQUEST_TIMEOUT,
                    _ => StatusCode::BAD_GATEWAY,
                }
            }
        }
        Ok(self)
    }

    pub async fn response_for(rw_resp: &RwLock<SharedResp>) -> Result<HttpResponse, Error> {
        {
            let cache = rw_resp.read().await;
            if cache.unexpired() {
                return Ok(HttpResponse::build(cache.status).body(cache.body.clone()));
            }
        }
        let mut cache = rw_resp.write().await;
        // 2nd check. might've raced with other write-wanting requests.
        if cache.unexpired() {
            return Ok(HttpResponse::build(cache.status).body(cache.body.clone()));
        }
        cache.update().await?;
        Ok(HttpResponse::build(cache.status).body(cache.body.clone()))
    }
}

#[get("/trains")]
async fn get_trains(cache_data: Data<HttpCaches>) -> Result<HttpResponse, Error> {
    SharedResp::response_for(&cache_data.get_ref().trains).await
}

#[get("/buses")]
async fn get_buses(cache_data: Data<HttpCaches>) -> Result<HttpResponse, Error> {
    SharedResp::response_for(&cache_data.get_ref().buses).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let http_caches = Data::new(HttpCaches::new());

    HttpServer::new(move || {
        App::new()
            .service(get_trains)
            .service(get_buses)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(http_caches.clone())
    })
    .workers(10)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
