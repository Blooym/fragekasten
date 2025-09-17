use anyhow::Result;
use chrono::Utc;
use sqlx::{
    SqlitePool, migrate, query,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode},
};
use std::{convert::Infallible, str::FromStr, sync::Arc, time::Duration};
use tokio::task::JoinHandle;
use tracing::{debug, warn};

pub type DatabasePool = SqlitePool;

pub struct Database {
    pool: DatabasePool,
}

impl Database {
    pub async fn new(connect_string: &str) -> Result<Self> {
        let pool = DatabasePool::connect_with(
            SqliteConnectOptions::from_str(connect_string)?
                .journal_mode(SqliteJournalMode::Wal)
                .auto_vacuum(SqliteAutoVacuum::Full)
                .optimize_on_close(true, None)
                .create_if_missing(true),
        )
        .await?;
        migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn spawn_cleanup_task(database: Arc<Database>) -> JoinHandle<Infallible> {
        tokio::spawn(async move {
            loop {
                debug!("Running ask expiry check");
                let now = Utc::now().timestamp();
                match query!("DELETE FROM asks WHERE deleteAfter < $1", now)
                    .execute(database.pool())
                    .await
                {
                    Ok(result) => {
                        debug!(
                            "Expiry check removed {} expired asks",
                            result.rows_affected()
                        )
                    }
                    Err(err) => {
                        warn!("Expiry check failed to remove expired asks: {err:?}");
                    }
                }
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        })
    }
}
