//! # Monobank Sync Rust
//!
//! A Rust application for synchronizing Monobank data with different database backends.
//!
//! ## Features
//!
//! - **SQLite Support**: Default database backend with file-based storage
//! - **PostgreSQL Support**: Enterprise database support with `--features postgres`
//! - **Offline Mode**: Build without database connection using `--features offline`
//! - **Async Workaround**: Synchronous API over async database operations
//!
//! ## Usage
//!
//! ### SQLite (Default)
//! ```bash
//! cargo build
//! DATABASE_URL=sqlite:./monobank.db ./target/debug/monobank-sync-rust
//! ```
//!
//! ### PostgreSQL
//! ```bash
//! cargo build --features postgres
//! DATABASE_URL=postgresql://user:pass@localhost/dbname ./target/debug/monobank-sync-rust
//! ```
//!
//! ### Offline Build (CI/CD)
//! ```bash
//! cargo build --features offline
//! ```

pub mod api;
pub mod config;
pub mod crud;
pub mod db;
pub mod db_types;
pub mod logger;
pub mod models;
pub mod schema;
pub mod utils;

pub use db::initialize;
pub use db_types::DatabasePool;
