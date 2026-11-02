use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::{debug, error};

use sqlx::{Arguments, Row};

// use crate::components::data_sources_view::node::Node;
// use crate::nodes::ConnectionSettings;
use crate::nodes::table_node::TableNode;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct SchemaTablesNode {
    schema_name: String,
    pool: sqlx::Pool<sqlx::MySql>,
}

impl SchemaTablesNode {
    pub fn new(pool: sqlx::Pool<sqlx::MySql>, schema_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            // settings: settings,
            pool: pool,
        };
    }

    async fn fetch_tables(&self) -> anyhow::Result<Vec<String>> {
        // let mut args = sqlx::mysql::MySqlArguments::default();
        // let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::new(
            "
        select \
            TABLE_NAME \
        from information_schema.tables \
        where \
          table_type = 'BASE TABLE'",
            // args,
        );
        let query = builder.build();

        // let pool = self.pool.await?;
        let results = query.fetch_all(&self.pool).await?;
        debug!("results: {:?}", results);
        let tables: Vec<String> = results
            .into_iter()
            // mysql is case sensitive
            .map(|r| r.get::<String, _>("TABLE_NAME"))
            .collect();
        return Ok(tables);
    }
}

#[async_trait]
impl Node for SchemaTablesNode {
    fn name(&self) -> &str {
        return "Tables";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_tables().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!("unable to fetch children async"));
            }
            Ok(tables) => {
                // let schema_name = self.schema_name.clone();
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for table in tables {
                    let boxed: Arc<dyn Node> =
                        Arc::new(TableNode::new(self.pool.clone(), &self.schema_name, &table));
                    result.push(boxed);
                }
                return Ok(Some(result));
            }
        }
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}
