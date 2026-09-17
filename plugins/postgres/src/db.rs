use std::cell::RefCell;
use std::sync::Arc;
use tracing::{debug, error, info};

use tokio::runtime::{Builder, Runtime};
use tokio::sync::{OnceCell, RwLock};

#[derive(Debug, Clone)]
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
        self.pw = Some(pw.clone());
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
    pub async fn add_connection(&self, setting: &ConnectionSettings) -> anyhow::Result<()> {
        info!("add_connection");

        let ConnectionSettings {
            name,
            db,
            host,
            port,
            user,
            pw,
        } = setting;
        if let Some(pw) = pw {
            let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");
            debug!("uri: {}", uri);

            let settings = self.settings.read().await;
            match settings.get(name) {
                Some(_s) => {
                    error!("connection already exists {}", name);
                    return Err(anyhow::anyhow!("connection already exists {}", name));
                }
                None => {
                    // drop read lock and obtain write lock
                    drop(settings);
                    let mut settings = self.settings.write().await;
                    settings.insert(name.clone(), setting.clone());

                    return Ok(());
                }
            }
        } else {
            return Err(anyhow::anyhow!("Password is not set"));
        }
    }

    pub async fn get_pool(&self, name: &str) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
        let pools = self.pools.read().await;
        match pools.get(name) {
            None => {
                // pool does not exist, try to create it
                let settings = self.settings.read().await;
                match settings.get(&name.to_string()) {
                    None => {
                        error!("connection settings do not exist {}", name);
                        return Err(anyhow::anyhow!("connection does not exist {}", name));
                    }
                    Some(s) => {
                        debug!("creating pool for {}", name);
                        let ConnectionSettings {
                            name,
                            db,
                            host,
                            port,
                            user,
                            pw,
                        } = s;
                        if let Some(pw) = pw {
                            let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");
                            debug!("creating pool for {} {} (2)", name, uri);
                            match sqlx::Pool::connect(&uri).await {
                                Err(e) => {
                                    error!("unable to create pool for {}: {}", name, e);
                                    return Err(anyhow::anyhow!(
                                        "unable to create pool for {}",
                                        name
                                    ));
                                }
                                Ok(pool) => {
                                    drop(pools);
                                    let mut pools = self.pools.write().await;
                                    pools.insert(name.clone(), pool.clone());
                                    return Ok(pool);
                                }
                            }
                        } else {
                            return Err(anyhow::anyhow!("Unable to create connection"));
                        }
                    }
                }
            }
            Some(pool) => {
                return Ok(pool.clone());
            }
        }
    }
}
