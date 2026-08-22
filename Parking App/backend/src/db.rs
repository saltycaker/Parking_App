use sqlx::postgres::PgPoolOptions;
use crate::error::AppResult;

#[derive(Clone)]
pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn new_with_retry(database_url: &str, max_retries: u32) -> AppResult<Self> {
        let mut retries = 0;
        let mut last_error = None;

        while retries < max_retries {
            match Self::new(database_url).await {
                Ok(db) => return Ok(db),
                Err(e) => {
                    last_error = Some(e);
                    retries += 1;
                    if retries < max_retries {
                        let wait_time = std::time::Duration::from_secs(2u64.pow(retries));
                        tracing::warn!("Database connection attempt {} failed, retrying in {}s...", retries, wait_time.as_secs());
                        tokio::time::sleep(wait_time).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| crate::error::AppError::Internal("Database connection failed after retries".to_string())))
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}
