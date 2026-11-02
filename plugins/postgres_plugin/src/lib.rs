mod node;

use silo_api::plugin::Plugin;
use sqlx::{Pool, postgres::PgPoolOptions};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tracing::{debug, error, info};

use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to start Tokio runtime"))
}

use silo_api::{
    Silo,
    data_sources::DataSource,
    node::{Node, NodeType},
};

const PLUGIN_NAME: &str = "postgres";

#[derive(Debug)]
pub struct PostgresPlugin {
    pub silo: RwLock<Option<Arc<dyn Silo>>>,
    pub sources: RwLock<HashMap<String, DataSource>>,
    pub connections: RwLock<HashMap<String, Pool<sqlx::Postgres>>>,
}

impl PostgresPlugin {
    pub fn new() -> Self {
        return Self {
            silo: RwLock::new(None),
            sources: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
        };
    }

    async fn create_connection(&self, source: &DataSource) {
        info!("PostgresPlugin::create_connection()");

        if let Ok(opt_silo) = self.silo.read() {
            if let Some(silo) = opt_silo.as_ref() {
                let silo_clone = Arc::clone(silo);
                let handle = get_runtime().spawn(async move {
                    let result = silo_clone.show_password_dialog().await;
                });

                // if let Ok(pw) = silo.show_password_dialog().await {
                // } else {
                // }
            } else {
                error!("PostgresPlugin::create_connection(): Silo object is empty");
            }
        } else {
            error!("PostgresPlugin::create_connection(): unable to obtain Silo object");
        }

        // let uri = format!(
        //     "postgres://{user}:{password}@{host}:{port}",
        //     user, pw, host, port
        // );
        // let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");
        // let pool = PgPoolOptions::new()
        //     .max_connections(5)
        //     .connect_lazy(s)
        //     .unwrap();
    }
}

#[async_trait::async_trait]
impl Plugin for PostgresPlugin {
    fn name(&self) -> String {
        return String::from(PLUGIN_NAME);
    }

    fn set_silo(&self, silo: Arc<dyn Silo>) {
        info!("PostgresPlugin::set_silo()");

        if let Ok(mut opt_silo) = self.silo.write() {
            *opt_silo = Some(silo);
        } else {
            error!("PostgresPlugin::set_silo(): unable to obtain write access to silo");
        }
    }

    fn initialize(&self, source: &DataSource) {
        info!("PostgresPlugin::initialize()");

        let dsn = source.data_source_name.clone();
        if let Ok(mut sources) = self.sources.write() {
            sources.insert(dsn, source.clone());
        }
    }

    async fn get_child_nodes(&self, node: &dyn Node) -> Vec<Arc<dyn Node>> {
        info!("PostgresPlugin::get_child_nodes()");

        let dsn = node.data_source_name().clone();

        // get connection
        if let Ok(cns) = self.connections.read() {
            // if let Some(cn) = cns.get(dsn) {
            //     todo!();
            // } else {
            //     // create connection
            // }
        } else {
            error!("PostgresPlugin::get_child_nodes(): unable obtain connections");
        }

        // todo!();
        return vec![];
    }
}
