#[derive(serde::Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub mongo: MongoConfig,
}

#[derive(serde::Deserialize)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16
}

#[derive(serde::Deserialize)]
pub struct NetConfig {
    pub host: String,
    pub port: u16
}