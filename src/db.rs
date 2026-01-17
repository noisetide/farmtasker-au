#![cfg(feature = "ssr")]

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub type DbPool = SqlitePool;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/farmtasker.sqlite".to_string());

    let _ = std::fs::create_dir_all("data");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
