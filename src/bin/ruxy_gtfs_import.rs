use ruxy::settings::{build as build_settings, migrated_pool};
use zip::read::ZipArchive; // needed for is_empty()

#[derive(Debug, serde::Deserialize)]
struct Trip {
    trip_id: u64,
    direction_id: u64,
    block_id: u64,
    shape_id: u64,
    route_id: u64,
    service_id: u64,
}

#[actix_web::main]
async fn main() {
    let settings = build_settings().unwrap();
    let db_pool = migrated_pool(&settings).await.unwrap();
    let file = gtfs_zip_file(&settings.gtfs_zip_url).await.unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let trips = archive.by_name(&"trips.txt").unwrap();
    let mut reader = csv::Reader::from_reader(trips);
    for result in reader.deserialize() {
        let trip: Trip = result.unwrap();
        let mut query = sqlx::query("select 1 from trips where trip_id = ?")
            .bind(trip.trip_id.to_string())
            .fetch_optional(&db_pool)
            .await
            .unwrap();
        if query.is_none() {
            println!("inserting {}", trip.trip_id);
            sqlx::query(
                "INSERT INTO trips (trip_id, direction_id, block_id, shape_id, route_id, service_id)
                 VALUES (?, ?, ?, ?, ?, ?)"
                ).bind(trip.trip_id.to_string())
                .bind(trip.direction_id.to_string())
                .bind(trip.block_id.to_string())
                .bind(trip.shape_id.to_string())
                .bind(trip.route_id.to_string())
                .bind(trip.service_id.to_string())
                .execute(&db_pool).await.unwrap();
        }
        break;
    }

    println!("done!");
}

async fn gtfs_zip_file(url: &str) -> Result<std::fs::File, std::io::Error> {
    let mut file = tempfile::tempfile()?;
    println!("getting: {}", url);
    //let mut res = awc::Client::default().get(url).send().await;
    let mut res = reqwest::get(url).await.unwrap();
    let mut bytes = res.bytes().await.unwrap();
    std::io::copy(&mut bytes.as_ref(), &mut file).unwrap();
    Ok(file)
}
