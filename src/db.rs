use crate::db_types::DatabasePool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

pub async fn initialize(database_url: &str) -> DatabasePool {
    #[cfg(feature = "postgres")]
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");

    #[cfg(feature = "sqlite")]
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to SQLite database");

    #[cfg(feature = "postgres")]
    sqlx::migrate!("./migrations/postgres")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    #[cfg(feature = "sqlite")]
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
