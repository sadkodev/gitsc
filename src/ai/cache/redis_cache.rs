use redis::{AsyncCommands, Client, RedisResult};

const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = con.get(key).await?;
        Ok(value)
    }

    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let _: () = con.set_ex(key, value, CACHE_TTL_SECONDS).await?;
        Ok(())
    }
}
