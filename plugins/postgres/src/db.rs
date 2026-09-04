use std::cell::RefCell;
use std::sync::Arc;
use tracing::{debug, error};

use tokio::runtime::{Builder, Runtime};
use tokio::sync::{OnceCell, RwLock};

pub struct ConnectionManager {
    pools: RwLock<std::collections::HashMap<String, sqlx::Pool<sqlx::Postgres>>>,
}

static CM: OnceCell<ConnectionManager> = OnceCell::const_new();

pub async fn get_connection_manager() -> &'static ConnectionManager {
    CM.get_or_init(|| async {
        ConnectionManager {
            pools: RwLock::new(std::collections::HashMap::new()),
        }
    })
    .await
}

impl ConnectionManager {
    pub async fn add_connection(
        &self,
        name: &String,
        db: &String,
        host: &String,
        port: u16,
        user: &String,
        pw: &String,
    ) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
        let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
        {
            Err(e) => {
                error!("unable to add connection: {}", e);
                return Err(anyhow::anyhow!("unable to add connection: {}", e));
            }
            Ok(pool) => {
                let mut pools = self.pools.write().await;
                pools.insert(name.to_string(), pool.clone());
                return Ok(pool.clone());
            }
        }
    }

    pub async fn get_pool(&self, name: &str) -> Option<sqlx::Pool<sqlx::Postgres>> {
        let pools = self.pools.read().await;
        pools.get(name).cloned()
    }
}
