#[derive(serde::Deserialize)]
pub struct Config {
    pub mongo: MongoConfig,
}

#[derive(serde::Deserialize)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16
}