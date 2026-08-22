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

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}
