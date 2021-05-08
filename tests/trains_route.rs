use actix_web::http::StatusCode;
use actix_web::{test, App};
use env_logger::Env;
use ruxy::http_caches::HttpCaches;
use ruxy::routes::get_trains;

#[actix_rt::test]
async fn trains_route_works() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mut app = test::init_service(
        App::new()
            .service(get_trains)
            .wrap(actix_web::middleware::Logger::default())
            .data(HttpCaches::default()),
    )
    .await;
    let req = test::TestRequest::get().uri("/trains").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
