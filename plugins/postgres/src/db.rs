use std::cell::RefCell;
use std::sync::Arc;
use tracing::{debug, error};

use tokio::runtime::{Builder, Runtime};
use tokio::sync::{OnceCell, RwLock};

#[derive(Debug)]
pub(super) struct ConnectionSettings {
    pub(super) name: String,
    pub(super) db: String,
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) user: String,
    pub(super) pw: Option<String>,
}

impl ConnectionSettings {
    pub fn set_pw(&mut self, pw: &String) {
        self.pw = pw.clone();
    }
}

#[derive(Debug)]
pub struct ConnectionManager {
    settings: RwLock<std::collections::HashMap<String, ConnectionSettings>>,
    pools: RwLock<std::collections::HashMap<String, sqlx::Pool<sqlx::Postgres>>>,
}

static CM: OnceCell<ConnectionManager> = OnceCell::const_new();

pub async fn get_connection_manager() -> &'static ConnectionManager {
    CM.get_or_init(|| async {
        ConnectionManager {
            settings: RwLock::new(std::collections::HashMap::new()),
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
        pw: Option<String>,
    ) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
        let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

        let settings = self.settings.read().await;
        match settings.get(name) {
            Some(s) => {
                error!("connection already exists {}", name);
                return Err(anyhow::anyhow!("connection already exists {}", name));
            }
            None => {
                let mut settings = self.settings.write().await;
                settings.insert(
                    name.clone(),
                    ConnectionSettings {
                        name: name.clone(),
                        db: db.clone(),
                        host: host.clone(),
                        port,
                        user: user.clone(),
                        pw: pw.clone(),
                    },
                );

                return Ok(());
            }
        }

        // match sqlx::postgres::PgPoolOptions::new()
        //     .max_connections(5)
        //     // .connect(&uri)
        //     .await
        // {
        //     Err(e) => {
        //         error!("unable to add connection: {}", e);
        //         return Err(anyhow::anyhow!("unable to add connection: {}", e));
        //     }
        //     Ok(pool) => {
        //         let mut pools = self.pools.write().await;
        //         pools.insert(name.to_string(), pool.clone());
        //         return Ok(pool.clone());
        //     }
        // }
    }

    pub async fn get_pool(&self, name: &str) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
        let settings = self.settings.read().await;
        match settings.get(name.to_string()) {
            None => {
                error!("connection does not exist {}", name);
                return Err(anyhow::anyhow!("connection does not exist {}", name));
            }
            Some(s) => {
                if s.pw.is_none() {
                    // pw is not set, ask from user
                } else {
                    let pools = self.pools.read().await;
                    match pools.get(name) {
                        None => {
                            error!("pool does not exist {}", name);
                            return Err(anyhow::anyhow!("pool does not exist {}", name));
                        }
                        Some(p) => {
                            return Ok(p.clone());
                        }
                    }
                }

                return Err(anyhow::anyhow!("//todo"));
            }
        }

        // let pools = self.pools.read().await;
        // pools.get(name).cloned()
    }
}
