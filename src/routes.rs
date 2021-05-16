use crate::http_caches::{HttpCaches, SharedResp};
use actix_web::HttpResponse;
use actix_web::{get, web::Data};
use awc::error::PayloadError;

#[get("/trains")]
pub async fn get_trains(cache_data: Data<HttpCaches>) -> Result<HttpResponse, PayloadError> {
    SharedResp::response_for(&cache_data.trains).await
}

#[get("/buses")]
pub async fn get_buses(cache_data: Data<HttpCaches>) -> Result<HttpResponse, PayloadError> {
    SharedResp::response_for(&cache_data.buses).await
}
