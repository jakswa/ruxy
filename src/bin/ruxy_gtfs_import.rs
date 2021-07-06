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
#[derive(Debug, serde::Deserialize)]
struct StopTime {
    trip_id: i64,
    arrival_time: String,
    departure_time: String,
    stop_id: i64,
    stop_sequence: i64,
}
#[derive(Debug, serde::Deserialize)]
struct Stop {
    stop_id: i64,
    stop_code: i64,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
}
#[derive(Debug, serde::Deserialize)]
struct Route {
    route_id: i64,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: i64,
    route_url: String,
    route_text_color: String,
}
#[derive(Debug, serde::Deserialize)]
struct CalendarDate {
    service_id: i64,
    date: String,
    exception_type: i64,
}
#[derive(Debug, serde::Deserialize)]
struct Calendar {
    service_id: i64,
    monday: i8,
    tuesday: i8,
    wednesday: i8,
    thursday: i8,
    friday: i8,
    saturday: i8,
    sunday: i8,
    start_date: String,
    end_date: String,
}

#[actix_web::main]
async fn main() {
    let settings = build_settings().unwrap();
    let mut conn = migrated_pool(&settings)
        .await
        .unwrap()
        .acquire()
        .await
        .unwrap();
    let file = gtfs_zip_file(&settings.gtfs_zip_url).await.unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    // guh. blocks here rather than proper cleanup. One day...

    {
        let calendar_dates = archive.by_name(&"calendar_dates.txt").unwrap();
        let mut reader = csv::Reader::from_reader(calendar_dates);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let cdate: CalendarDate = result.unwrap();
            insert_cdate(&mut conn, cdate).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }
    {
        let calendar = archive.by_name(&"calendar.txt").unwrap();
        let mut reader = csv::Reader::from_reader(calendar);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let cal: Calendar = result.unwrap();
            insert_calendar(&mut conn, cal).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }
    {
        let routes = archive.by_name(&"routes.txt").unwrap();
        let mut reader = csv::Reader::from_reader(routes);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let route: Route = result.unwrap();
            insert_route(&mut conn, route).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }
    {
        let trips = archive.by_name(&"trips.txt").unwrap();
        let mut reader = csv::Reader::from_reader(trips);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let trip: Trip = result.unwrap();
            insert_trip(&mut conn, trip).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }
    {
        let stop_times = archive.by_name(&"stop_times.txt").unwrap();
        let mut reader = csv::Reader::from_reader(stop_times);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let stop_time: StopTime = result.unwrap();
            insert_stop_time(&mut conn, stop_time).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }
    {
        let stops = archive.by_name(&"stops.txt").unwrap();
        let mut reader = csv::Reader::from_reader(stops);
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        for result in reader.deserialize() {
            let stop: Stop = result.unwrap();
            insert_stop(&mut conn, stop).await;
        }
        sqlx::query!("COMMIT").execute(&mut conn).await.unwrap();
    }

    println!("done!");
}

async fn insert_calendar(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, cal: Calendar) {
    sqlx::query!(
        "INSERT INTO calendar (service_id, monday, tuesday, wednesday, thursday, friday, saturday, sunday, start_date, end_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        cal.service_id,
        cal.monday,
        cal.tuesday,
        cal.wednesday,
        cal.thursday,
        cal.friday,
        cal.saturday,
        cal.sunday,
        cal.start_date,
        cal.end_date,
    ).execute(conn).await.ok();
}
async fn insert_cdate(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, cdate: CalendarDate) {
    sqlx::query!(
        "INSERT INTO calendar_dates (service_id, date, exception_type) VALUES (?, ?, ?)",
        cdate.service_id,
        cdate.date,
        cdate.exception_type,
    )
    .execute(conn)
    .await
    .ok();
}
async fn insert_route(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, route: Route) {
    sqlx::query!(
        "INSERT INTO routes (route_id, route_short_name, route_long_name, route_desc, route_type, route_url, route_text_color) VALUES (?, ?, ?, ?, ?, ?, ?)",
        route.route_id,
        route.route_short_name,
        route.route_long_name,
        route.route_desc,
        route.route_type,
        route.route_url,
        route.route_text_color
    ).execute(conn).await.ok();
}
async fn insert_stop(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, stop: Stop) {
    sqlx::query!(
        "INSERT INTO stops (stop_id, stop_code, stop_name, stop_lat, stop_lon) VALUES (?, ?, ?, ?, ?)",
        stop.stop_id,
        stop.stop_code,
        stop.stop_name,
        stop.stop_lat,
        stop.stop_lon
    ).execute(conn).await.ok();
}
async fn insert_stop_time(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
    stop_time: StopTime,
) {
    sqlx::query!(
        "INSERT INTO stop_times (trip_id, arrival_time, departure_time, stop_id, stop_sequence) VALUES (?, ?, ?, ?, ?)",
        stop_time.trip_id,
        stop_time.arrival_time,
        stop_time.departure_time,
        stop_time.stop_id,
        stop_time.stop_sequence
    ).execute(conn).await.ok();
}
async fn insert_trip(conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>, trip: Trip) {
    sqlx::query!(
        "INSERT INTO trips (trip_id, direction_id, block_id, shape_id, route_id, service_id) VALUES (?, ?, ?, ?, ?, ?)",
        trip.trip_id,
        trip.direction_id,
        trip.block_id,
        trip.shape_id,
        trip.route_id,
        trip.service_id
    ).execute(conn).await.ok();
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


