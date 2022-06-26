use serde::Deserialize;

use std::path::Path;
pub fn parse_config() -> Config {
    let mut args = std::env::args().skip(1);
    let config = match args.next() {
        Some(cfg_path) => {
            use std::fs::*;
            match read(Path::new(&cfg_path)) {
                Ok(file) => {
                    toml::from_slice::<Config>(&file).unwrap()
                },
                Err(e) => {
                    println!("{}", e);
                    panic!("fail read config file")
                },
            }
        }
        _ => {
            panic!("no config path")
        }
    };
    config
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub cors: CorsConfig
}


#[derive(Debug, Deserialize)]
pub struct CorsConfig {
    pub origins: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct NetConfig {
    pub host: String,
    pub port: u16,
}