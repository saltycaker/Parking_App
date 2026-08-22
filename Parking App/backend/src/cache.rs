use deadpool_redis::{Config, Pool, Runtime};
use redis::AsyncCommands;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct Cache {
    pool: deadpool_redis::Pool,
}

impl Cache {
    pub async fn new(redis_url: &str) -> AppResult<Self> {
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        
        Ok(Cache { pool })
    }

    pub async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let mut conn = self.pool.get().await?;
        let value: Option<String> = conn.get(key).await?;
        Ok(value)
    }

    pub async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> AppResult<()> {
        let mut conn = self.pool.get().await?;
        conn.set_ex::<_, _, ()>(key, value, ttl_seconds).await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> AppResult<()> {
        let mut conn = self.pool.get().await?;
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.pool.get().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }
}
