extern crate config;
use config::{Environment, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub web_port: u16,
    pub ssl_cert: String,
    pub ssl_key: String,
    pub bus_url: String,
    pub train_url: String,
    pub train_api_key: String,
    pub database_url: String,
    pub gtfs_zip_url: String,
}

pub fn build() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("Settings").required(false))?
        .merge(Environment::with_prefix("RUXY"))?;

    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        settings.set("database_url", db_url)?;
    }

    settings.set(
        "train_url",
        format!(
            "{}?apikey={}",
            settings.get_str("train_base_url")?,
            settings.get_str("train_api_key")?
        ),
    )?;

    settings.try_into()
}

pub async fn migrated_pool(
    settings: &Settings,
) -> Result<sqlx::sqlite::SqlitePool, config::ConfigError> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&settings.database_url)
        .await
        .expect("DB connect failed");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migration failed :(");
    Ok(pool)
}
