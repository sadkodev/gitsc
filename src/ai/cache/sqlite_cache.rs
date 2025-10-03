use crate::error::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_rusqlite::Connection;

const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

pub struct SqliteCache {
    conn: Connection,
}

impl SqliteCache {
    pub async fn new(cache_path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(cache_path).await?;
        conn.call(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS cache (\n                    key TEXT PRIMARY KEY,\n                    value TEXT NOT NULL,\n                    timestamp INTEGER NOT NULL\n                )",
                [],
            )?;
            Ok(())
        }).await?;
        Ok(Self { conn })
    }
}

#[async_trait]
impl super::CacheRepository for SqliteCache {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let key = key.to_string();
        let result = self
            .conn
            .call(move |conn| {
                let mut stmt = conn.prepare("SELECT value, timestamp FROM cache WHERE key = ?")?;
                let mut rows = stmt.query([&key])?;

                if let Some(row) = rows.next()? {
                    let value: String = row.get(0)?;
                    let timestamp: u64 = row.get(1)?;
                    let current_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if current_time - timestamp < CACHE_TTL_SECONDS {
                        Ok(Some(value))
                    } else {
                        // Cache expired, delete it
                        conn.execute("DELETE FROM cache WHERE key = ?", [&key])?;
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            })
            .await?;
        Ok(result)
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let key = key.to_string();
        let value = value.to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.conn
            .call(move |conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO cache (key, value, timestamp) VALUES (?, ?, ?)",
                    [&key, &value, &timestamp.to_string()],
                )?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
