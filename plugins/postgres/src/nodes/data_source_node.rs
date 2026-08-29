use tracing::{debug, error};

use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};

use async_trait::async_trait;
use std::sync::Arc;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{PgPool, Row};

use crate::nodes::ConnectionSettings;
use crate::nodes::schema_node::SchemaNode;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct PostgresDataSourceNode {
    name: String,

    // sender: async_channel::Sender<String>,
    // receiver: async_channel::Receiver<String>,
    settings: Arc<ConnectionSettings>,
}

impl PostgresDataSourceNode {
    pub fn new(name: &str, settings: ConnectionSettings) -> Self {
        // let (sender, receiver) = async_channel::unbounded::<String>();

        return Self {
            name: name.to_string(),
            // sender,
            // receiver,
            settings: Arc::new(settings),
        };
    }

    fn fetch_schemas(&self) -> Result<Vec<String>, &'static str> {
        let user = self.settings.user.clone();
        let pw = self.settings.pw.clone();
        let host = self.settings.host.clone();
        let port = self.settings.port.clone();
        let db = self.settings.name.clone();

        let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

        let rt = crate::get_runtime();
        return rt.block_on(async {
            debug!("fetching schemas for uri: {}", uri);

            match PgPoolOptions::new().max_connections(1).connect(&uri).await {
                Err(e) => {
                    error!(
                        "an error occured while trying to connect to the database: {}",
                        e
                    );
                    return Err("an error occured while trying to connect to the databse");
                }
                Ok(pool) => {
                    let sql = "select schema_name from information_schema.schemata";

                    match sqlx::query(sql).fetch_all(&pool).await {
                        Err(e) => {
                            error!("an error occured while fetching schemas: {}", e);
                            return Err("an error occured while fetching schemas");
                        }
                        Ok(results) => {
                            let schemas: Vec<String> = results
                                .into_iter()
                                .map(|r| r.get::<String, _>("schema_name"))
                                .collect();
                            return Ok(schemas);
                        }
                    }
                }
            }
        });
    }

    async fn fetch_schemas_async(&self) -> Result<Vec<String>, &'static str> {
        let user = self.settings.user.clone();
        let pw = self.settings.pw.clone();
        let host = self.settings.host.clone();
        let port = self.settings.port.clone();
        let db = self.settings.name.clone();

        let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

        match PgPoolOptions::new().max_connections(1).connect(&uri).await {
            Err(e) => {
                error!(
                    "an error occured while trying to connect to the database: {}",
                    e
                );
                return Err("an error occured while trying to connect to the databse");
            }
            Ok(pool) => {
                let sql = "select schema_name from information_schema.schemata";

                match sqlx::query(sql).fetch_all(&pool).await {
                    Err(e) => {
                        error!("an error occured while fetching schemas: {}", e);
                        return Err("an error occured while fetching schemas");
                    }
                    Ok(results) => {
                        let schemas: Vec<String> = results
                            .into_iter()
                            .map(|r| r.get::<String, _>("schema_name"))
                            .collect();
                        return Ok(schemas);
                    }
                }
            }
        }
    }
}

#[async_trait]
impl Node for PostgresDataSourceNode {
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    // fn clone_box(&self) -> Box<dyn Node> {
    //     debug!("PostgresDataSourceNode::clone_box");

    //     let boxed: Box<dyn Node> = Box::new(self.clone());
    //     return boxed;
    // }

    fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        debug!("PostgresDataSourceNode::children");

        match self.fetch_schemas() {
            Err(e) => {
                error!("unable to fetch schemas: {}", e);
                return None;
            }
            Ok(schemas) => {
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for schema in schemas {
                    let arced: Arc<dyn Node> =
                        Arc::new(SchemaNode::new(schema.as_str(), Arc::clone(&self.settings)));
                    result.push(arced);
                }
                return Some(result);
            }
        }
    }

    async fn children_async(&self) -> Result<Option<Vec<Arc<dyn Node>>>, &'static str> {
        debug!("PostgresDataSourceNode::children");

        match self.fetch_schemas_async().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err("unable to fetch schemas");
            }
            Ok(schemas) => {
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for schema in schemas {
                    let boxed: Arc<dyn Node> =
                        Arc::new(SchemaNode::new(schema.as_str(), Arc::clone(&self.settings)));
                    result.push(boxed);
                }
                return Ok(Some(result));
            }
        }
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Edit"), Some("win.data-source-edit::postgres"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);

        let item = gio::MenuItem::new(
            Some("New Query"),
            Some("win.data-source-new-query::postgres"),
        );
        let section = gio::Menu::new();
        section.append_item(&item);

        menu.append_section(None, &section);

        let item = gio::MenuItem::new(Some("Remove"), Some("win.data-source-remove::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return Some(menu);
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return Some(Arc::new(self.clone()));
    }
}

#[async_trait]
impl DataSourceNode for PostgresDataSourceNode {
    async fn query(&self, sql: &str) -> Result<(), &'static str> {
        debug!("PostgresDataSourceNode::query {}", sql);
        return Ok(());
    }
}
