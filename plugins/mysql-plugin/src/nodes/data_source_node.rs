use async_trait::async_trait;
use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use std::sync::Arc;
use tokio::sync::OnceCell;

use tracing::{debug, error};

use crate::nodes::ConnectionSettings;
use crate::nodes::schema_functions_node::SchemaFunctionsNode;
use crate::nodes::schema_procedures_node::SchemaProceduresNode;
use crate::nodes::schema_tables_node::SchemaTablesNode;
use silo_plugin::node::{DataSourceNode, Node, QueryResult};

#[derive(Debug, Clone)]
pub struct MySQLDataSourceNode {
    name: String,
    // settings: Arc<ConnectionSettings>,
    // pool: OnceCell<Pool<MySql>>,
    pool: sqlx::mysql::MySqlPool,
}

impl MySQLDataSourceNode {
    pub fn new(name: &str, pool: &sqlx::mysql::MySqlPool) -> Self {
        return Self {
            name: name.to_string(),
            // settings: Arc::new(settings),
            pool: pool.clone(),
        };
    }

    // async fn get_pool(&self) -> anyhow::Result<&Pool<MySql>> {
    //     match self
    //         .pool
    //         .get_or_try_init(|| async {
    //             let user = self.settings.user.clone();
    //             let pw = self.settings.pw.clone();
    //             let host = self.settings.host.clone();
    //             let port = self.settings.port.clone();
    //             let db = self.settings.db.clone();

    //             let uri = format!("mysql://{user}:{pw}@{host}:{port}/{db}");

    //             match MySqlPoolOptions::new()
    //                 .max_connections(5)
    //                 .connect(&uri)
    //                 .await
    //             {
    //                 Err(e) => {
    //                     error!("unable to connect to database: {}", e);
    //                     return Err(anyhow::anyhow!("unable to connect to database: {}", e));
    //                 }
    //                 Ok(pool) => {
    //                     return Ok(pool);
    //                 }
    //             }
    //         })
    //         .await
    //     {
    //         Err(e) => {
    //             // return Err(anyhow::anyhow!(PostgresError::ConnectionError(e)));
    //             return Err(e);
    //         }
    //         Ok(pool) => {
    //             return Ok(pool);
    //         }
    //     }
    // }
}

#[async_trait]
impl Node for MySQLDataSourceNode {
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        let mut nodes: Vec<Arc<dyn Node>> = vec![];

        let boxed: Arc<dyn Node> = Arc::new(SchemaTablesNode::new(self.pool.clone(), &self.name));
        nodes.push(boxed);

        return Ok(Some(nodes));
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Edit"), Some("win.data-source-edit::mysql"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::mysql"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        let item = gio::MenuItem::new(Some("Remove"), Some("win.data-source-remove::mysql"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return Some(menu);
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return Some(Arc::new(self.clone()));
    }
}

#[async_trait]
impl DataSourceNode for MySQLDataSourceNode {
    async fn query(&self, sql: &str) -> anyhow::Result<QueryResult> {
        return Err(anyhow::anyhow!("//todo not implemented"));
    }

    fn get_configuration(&self) -> serde_json::Value {
        return serde_json::json!({
            "name": self.name.clone(),
            // "host": self.settings.host.clone(),
            // "port": self.settings.port.clone(),
            // "user": self.settings.user.clone(),
            // "pw": self.settings.pw.clone(),
        });
    }
}
