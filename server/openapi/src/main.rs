mod config;
mod filter;
mod common;
mod error;
use warp::Filter;

mod danmaku;
mod superchat;

use std::{net::{Ipv4Addr, Ipv6Addr, IpAddr, SocketAddr}, path::Path, fmt::format};

fn parse_config() -> config::Config {
    let mut args = std::env::args().skip(1);
    let config = match args.next() {
        Some(cfg_path) => {
            use std::fs::*;
            match read(Path::new(&cfg_path)) {
                Ok(file) => {
                    toml::from_slice::<config::Config>(&file).unwrap()
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

#[tokio::main]
async fn main() {
    
    use mongodb::options::*;
    println!("db connecting ");
    let config = parse_config();
    let host = config.mongo.host;
    let port = Some(config.mongo.port);
    let net_addr = format!("{}:{}", config.net.host, config.net.port).parse::<SocketAddr>().expect("invalid net config");
    let options = ClientOptions::builder().hosts(vec![ServerAddress::Tcp{host, port}]).build();
    let db = mongodb::Client::with_options(options).map(|client| {
        client.database(config.mongo.db.as_str())
    }).unwrap();

    let db_clone = db.clone();
    let danmaku = 
    warp::path!("danmaku" / u64)
    .and(warp::query::<danmaku::Query>())
    .and(warp::any().map(move || db_clone.clone()))
    .and_then(|roomid:u64, f, db:mongodb::Database| async move {
        let coll = db.collection(roomid.to_string().as_str());
        danmaku::query(f, coll).await
        .map(|r|warp::reply::json(&r))
        .map_err(|e| warp::reject::custom::<error::Error>(e.into()))
    });

    let db_clone = db.clone();
    let superchat = 
    warp::path!("sc" / u64)
    .and(warp::query::<superchat::Query>())
    .and(warp::any().map(move || db_clone.clone()))
    .and_then(|roomid:u64, f, db:mongodb::Database| async move {
        let coll = db.collection(roomid.to_string().as_str());
        superchat::query(f, coll).await
        .map(|r|warp::reply::json(&r))
        .map_err(|e| warp::reject::custom::<error::Error>(e.into()))
    });

    let route = danmaku.or(superchat);
    warp::serve(route)
        .run(net_addr)
        .await;
}
