use sqlx::pool::PoolOptions;
use sqlx::sqlite::SqlitePool;

pub async fn create_connection_pool() -> SqlitePool {
    PoolOptions::new().max_connections(5).connect(
        "sqlite://src/db/local.db"
    )
        .await.expect("Failed to connect to SQLite")
}