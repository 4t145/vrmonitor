use tokio::time::*;
use tokio::sync::RwLock;
pub struct RequestProxy<Args> {
    next_allow: RwLock<Instant>,
    cooldown: Duration,
    client: reqwest::Client,
    req_maker: fn(args: &Args) -> reqwest::Request
}

impl<Args> RequestProxy<Args> {
    pub fn new(cooldown: Duration, req_maker: fn(args: &Args) -> reqwest::Request) -> Self {
        let client = reqwest::Client::new();
        Self { next_allow: RwLock::new(Instant::now()), cooldown, client, req_maker}
    }

    pub async fn req(&self, args: &Args) -> reqwest::Result<reqwest::Response> {
        let next_allow = {
            self.next_allow.read().await.clone()
        };
        sleep_until(next_allow).await;
        let request = (self.req_maker)(args);
        {
            *self.next_allow.write().await = next_allow + self.cooldown;
        };
        self.client.execute(request).await
    }
}