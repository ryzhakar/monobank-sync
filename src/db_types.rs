#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

#[cfg(feature = "postgres")]
pub type DatabasePool = PgPool;
#[cfg(feature = "sqlite")]
pub type DatabasePool = SqlitePool;

