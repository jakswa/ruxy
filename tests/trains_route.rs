use actix_web::http::StatusCode;
use actix_web::{test, App};
use ruxy::http_caches::HttpCaches;
use ruxy::routes::{get_buses, get_trains};

// actually makes an HTTP request to the Settings.toml URL
#[actix_web::rt::test]
async fn trains_route_works() {
    let mut app =
        test::init_service(App::new().service(get_trains).data(HttpCaches::default())).await;
    let req = test::TestRequest::get().uri("/trains").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

// actually makes an HTTP request to the Settings.toml URL
#[actix_web::rt::test]
async fn buses_route_works() {
    let mut app =
        test::init_service(App::new().service(get_buses).data(HttpCaches::default())).await;
    let req = test::TestRequest::get().uri("/buses").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
