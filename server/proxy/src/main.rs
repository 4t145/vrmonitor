mod config;
mod cache;
mod proxy;


use std::sync::Arc;
use tokio::time;
use std::net::{SocketAddr};
use cache::Cache;
use proxy::RequestProxy;
use warp::*;

#[tokio::main]
async fn main() {
    
    let config = config::parse_config();
    let addr = format!("{}:{}", config.net.host, config.net.port).parse::<SocketAddr>().expect("invalid net config");
    let origins = config.cors.origins.iter().map(|x|x.as_str()).collect::<Vec<&str>>();
    let cors = warp::cors().allow_method("GET");
    let cors = if origins.contains(&"*") {
        cors.allow_any_origin()
    } else {
        cors.allow_origins(origins)
    };
    let cors = cors.build();
    let json = reply::with::header("content-type", "application/json");

    let roominfo_proxy = RequestProxy::new(time::Duration::from_millis(500), roominfo_req_maker);
    let roominfo_cache = Arc::new(Cache::new(roominfo_proxy, time::Duration::from_secs(30)));
    let with_roominfo_cache = any().map(move||roominfo_cache.clone());
    
    let userinfo_proxy = RequestProxy::new(time::Duration::from_millis(500), userinfo_req_maker);
    let userinfo_cache = Arc::new(Cache::new(userinfo_proxy, time::Duration::from_secs(3600)));
    let with_userinfo_cache = any().map(move||userinfo_cache.clone());
    
    let room_info = warp::path!("roominfo"/u64)
        .and(with_roominfo_cache)
        .and_then(fetch::<u64>)
        .with(json.clone())
        .with(cors.clone());

    let user_info = warp::path!("userinfo"/u64)
        .and(with_userinfo_cache)
        .and_then(fetch::<u64>)
        .with(json)
        .with(cors);

    let route = room_info.or(user_info);
    serve(route).bind(addr).await;
}

#[derive(Debug)]
pub enum Error {
    // IllegalUrl(url::ParseError),
    CannotBase,
    ParseError(&'static str),
    NoQuery,
    Http(warp::http::Error),
    Reqwest(reqwest::Error)
}

impl reject::Reject for Error {

}

async fn fetch<T: std::hash::Hash + Eq >(key: T, cache: Arc<Cache<T>>) -> Result<Vec<u8>, Rejection> {
    cache.get(key).await
    // .map(|r|r)
    .map_err(|e| warp::reject::custom::<Error>(Error::Reqwest(e)))
}

fn userinfo_req_maker(uid: &u64) -> reqwest::Request {
    let url = format!("https://api.bilibili.com/x/space/acc/info?mid={uid}");
    reqwest::Request::new(reqwest::Method::GET, reqwest::Url::parse(&url).unwrap())
}

fn roominfo_req_maker(roomid: &u64) -> reqwest::Request {
    let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={roomid}");
    reqwest::Request::new(reqwest::Method::GET, reqwest::Url::parse(&url).unwrap())
}
