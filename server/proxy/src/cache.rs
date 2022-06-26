use tokio::{sync::RwLock, time::{Instant, Duration}};
use std::collections::HashMap;
use crate::proxy::RequestProxy;


pub struct Cache<K> {
    cache: RwLock<HashMap<K, TimerCache<Vec<u8>>>>,
    proxy: RequestProxy<K>,
    expiration: Duration
}

impl<K: std::hash::Hash+Eq> Cache<K> {
    pub fn new(proxy: RequestProxy<K>, expiration: Duration) -> Self {
        let cache = RwLock::new(HashMap::new());
        Self {cache, proxy, expiration}
    }

    pub async fn get(&self, key: K) -> Result<Vec<u8>, reqwest::Error> {
        {
            let cache = self.cache.read().await;
            if let Some(timer_cache) = cache.get(&key) {
                if let Some(v) = timer_cache.get() {
                    let res = v.clone();
                    return Ok(res);
                } 
            } 
        }
        let res = self.proxy.req(&key).await?.bytes().await?.to_vec();
        let ret = Ok(res.clone());
        {
            let mut cache = self.cache.write().await;
            if let Some(timer_cache) = cache.get_mut(&key) {
                timer_cache.set(res);
            } else {
                cache.insert(key, TimerCache::new(res, self.expiration));
            }
        }
        ret
    }

    
}

pub struct TimerCache<T> {
    update_time: Instant,
    expiration: Duration,
    val: T
}

impl<T> TimerCache<T> {
    pub fn new(val:T, expiration: Duration) -> Self {
        Self { update_time: Instant::now(), expiration, val }
    }
    pub fn get(&self) -> Option<&'_ T> {
        (Instant::now() < self.update_time + self.expiration).then(||&self.val)
    }
    pub fn set(&mut self, val: T) -> T {
        let mut val = val;
        std::mem::swap(&mut val, &mut self.val);
        self.update_time = Instant::now();
        val
    }
}