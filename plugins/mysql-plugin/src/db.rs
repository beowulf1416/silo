use tracing::debug;

use tokio::runtime::{Builder, Runtime};
use tokio::sync::OnceLock;

struct ConnectionManager {
    pools: std::collections::HashMap<String, sqlx::Pool<sqlx::MySql>>,
}

static CM: OnceLock<ConnectionManager> = OnceLock::new();

fn get_connection_manager() -> &'static ConnectionManager {
    // RUNTIME.get_or_init(|| Runtime::new().expect("Failed to start Tokio runtime"))
    CM.get_or_init(|| ConnectionManager {
        pools: std::collections::HashMap::new(),
    })
    .expect("Failed to get connection manager")
}

impl ConnectionManager {
    pub fn add_connection(&mut self, name: &str, host: &str, port: u16, user: &str) {
        debug!("add_connection: name={name}, host={host}, port={port}, user={user}");
    }

    pub fn get_pool(&self, name: &str) {
        debug!("get_pool: name={name}");
    }
}
