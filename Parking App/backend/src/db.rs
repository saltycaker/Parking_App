use deadpool_postgres::{Config, Pool, Runtime};
use sqlx::postgres::PgPoolOptions;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
