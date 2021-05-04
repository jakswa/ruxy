extern crate config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub web_port: u16,
    pub ssl_cert: String,
    pub ssl_key: String,
    pub bus_url: String,
    pub train_url: String,
    pub train_api_key: String,
}

pub fn build() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();

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
