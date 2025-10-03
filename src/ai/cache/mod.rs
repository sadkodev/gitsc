use crate::error::Result;
use async_trait::async_trait;

pub mod diff_hasher;

pub mod sqlite_cache;

#[async_trait]
pub trait CacheRepository: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
}
