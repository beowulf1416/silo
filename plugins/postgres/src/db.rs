use tracing::{debug, error};

// use std::sync::OnceLock;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use tokio::runtime::{Builder, Runtime};
use tokio::sync::OnceLock;

enum DbResult {
    FetchSchemas(Vec<String>),
}

struct DbManager {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub user: String,
    pub pw: String,

    pub pool: PgPool,
    pub runtime: Runtime,
}

static DB: OnceLock<DbManager> = OnceLock::new();

pub fn build_db_manager(
    name: &str,
    host: &str,
    port: u32,
    user: &str,
    pw: &str,
) -> Result<(), &'static str> {
    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_all()
    //     .build()?;

    match Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
    {
        Err(e) => {
            error!("Failed to create runtime: {}", e);
            return Err("Failed to create runtime");
        }
        Ok(runtime) => {
            match runtime.block_on(async {
                match PgPoolOptions::new()
                    .max_connections(4)
                    .connect(format!("postgres://{user}:{pw}@{host}:{port}/{db}").as_str())
                    .await
                {
                    Err(e) => {
                        error!("unable to connect to database: {}", e);
                        return Err("Failed to connect to database");
                    }
                    Ok(pool) => Ok(pool),
                }
            }) {
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                    return Err("Failed to connect to database");
                }
                Ok(pool) => {
                    DB.set(DbManager {
                        name: name.to_string(),
                        host: host.to_string(),
                        port,
                        user: user.to_string(),
                        pw: pw.to_string(),
                        pool,
                        runtime,
                    });

                    return Ok(());
                }
            }
        }
    }
}

pub fn fetch_schemas() -> Result<Vec<String>, &'static str> {}
