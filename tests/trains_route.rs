use actix_web::http::StatusCode;
use actix_web::{test, App};
use ruxy::http_caches::HttpCaches;
use ruxy::routes::{get_buses, get_trains};

// actually makes an HTTP request to the Settings.toml URL
#[actix_web::rt::test]
async fn trains_route_works() {
    let cache = actix_web::web::Data::new(HttpCaches::default());
    let mut app = test::init_service(App::new().service(get_trains).app_data(cache.clone())).await;
    let req = test::TestRequest::get().uri("/trains").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

// actually makes an HTTP request to the Settings.toml URL
#[actix_web::rt::test]
async fn buses_route_works() {
    let cache = actix_web::web::Data::new(HttpCaches::default());
    let mut app = test::init_service(App::new().service(get_buses).app_data(cache.clone())).await;
    let req = test::TestRequest::get().uri("/buses").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}


