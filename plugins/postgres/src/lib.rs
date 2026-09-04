mod components;
mod db;
mod nodes;
pub mod plugin;

// use tracing::error;
use thiserror::Error;

use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to start Tokio runtime"))
}

#[derive(Debug, Error)]
pub enum PostgresError {
    #[error("failed to connect to database")]
    ConnectionError(#[from] sqlx::Error),
    #[error("failed to fetch schemas")]
    SchemaError,
    #[error("failed to execute query")]
    QueryError(sqlx::Error),
}
