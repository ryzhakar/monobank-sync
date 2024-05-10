use sqlx::SqlitePool;
use tokio::runtime::{Builder, Runtime};

pub async fn initialize(database_url: &str) -> SqlitePool {
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    pool
}

pub fn get_tokio_runtime() -> Runtime {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime")
}

pub struct DBExecutor {
    pub pool: SqlitePool,
    pub runtime: Runtime,
}

impl DBExecutor {
    pub fn execute<F, T>(&self, operation: F) -> T::Output
    where
        F: FnOnce(&SqlitePool) -> T + Send + 'static,
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        self.runtime.block_on(operation(&self.pool))
    }
}
