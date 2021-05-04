use actix_web::client::Client;
use actix_web::web::Bytes;
use actix_web::{http::StatusCode, Error, HttpResponse};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::settings;

pub struct HttpCaches {
    pub trains: RwLock<SharedResp>,
    pub buses: RwLock<SharedResp>,
}

impl Default for HttpCaches {
    fn default() -> Self {
        let settings = settings::build().unwrap();
        HttpCaches {
            trains: RwLock::new(SharedResp::new_blank(settings.train_url)),
            buses: RwLock::new(SharedResp::new_blank(settings.bus_url)),
        }
    }
}

#[derive(Debug)]
pub struct SharedResp {
    expiry: Instant,
    ttl: Duration,
    url: String,
    body: Bytes,
    status: StatusCode,
}

impl SharedResp {
    pub fn new_blank(url: String) -> SharedResp {
        SharedResp {
            url,
            expiry: Instant::now(),
            ttl: Duration::from_secs(10),
            body: Bytes::from(&b"<unused>"[..]),
            status: StatusCode::OK,
        }
    }

    fn unexpired(&self) -> bool {
        self.expiry > Instant::now()
    }

    async fn update(&mut self) -> Result<&SharedResp, Error> {
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

    // Multiple threads fight to safely read/write these cached HTTP responses.
    // An alternative was Mutex, but RwLock is hoping for read-heavy API usage.
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
