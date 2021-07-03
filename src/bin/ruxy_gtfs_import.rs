use ruxy::settings::{build as build_settings, migrated_pool};
use zip::read::ZipArchive; // needed for is_empty()

#[derive(Debug, serde::Deserialize)]
struct Trip {
    trip_id: i64,
    direction_id: i64,
    block_id: i64,
    shape_id: i64,
    route_id: i64,
    service_id: i64,
}

#[actix_web::main]
async fn main() {
    let settings = build_settings().unwrap();
    let mut conn = migrated_pool(&settings).await.unwrap()
        .acquire().await.unwrap();
    let file = gtfs_zip_file(&settings.gtfs_zip_url).await.unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let trips = archive.by_name(&"trips.txt").unwrap();
    let mut reader = csv::Reader::from_reader(trips);
    sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
    for result in reader.deserialize() {
        let trip: Trip = result.unwrap();
        insert_trip(&mut conn, trip).await;
    }
    sqlx::query("COMMIT").execute(&mut conn).await.unwrap();

    println!("done!");
}

async fn insert_trip(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, trip: Trip) {
    
    sqlx::query("INSERT INTO trips (trip_id, direction_id, block_id, shape_id, route_id, service_id) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(trip.trip_id)
        .bind(trip.direction_id)
        .bind(trip.block_id)
        .bind(trip.shape_id)
        .bind(trip.route_id)
        .bind(trip.service_id)
        .execute(conn).await.ok();
}

async fn gtfs_zip_file(url: &str) -> Result<std::fs::File, std::io::Error> {
    let mut file = tempfile::tempfile()?;
    println!("getting: {}\n", url);
    let res = reqwest::get(url).await.unwrap();
    let bytes = res.bytes().await.unwrap();
    std::io::copy(&mut bytes.as_ref(), &mut file).unwrap();
    println!("got it.");
    Ok(file)
}
