use ruxy::settings::{build as build_settings, migrated_pool};

#[actix_web::main]
async fn main() {
    let settings = build_settings().unwrap();
    let db_pool = migrated_pool(&settings).await.unwrap();
    gtfs_zip_file(&settings.gtfs_zip_url).await.unwrap();
    println!("done!");
}

async fn gtfs_zip_file(url: &str) -> Result<std::fs::File, std::io::Error> {
    let mut file = tempfile::tempfile()?;
    println!("getting: {}", url);
    let mut res = awc::Client::default().get(url).send().await.unwrap();
    println!("hmm: {:?}", res);
    Ok(file)
}
