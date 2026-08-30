use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;
use crate::nodes::table_node::TableNode;

#[derive(Debug, Clone)]
pub struct SchemaTablesNode {
    schema_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SchemaTablesNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            // settings: settings,
            pool: pool,
        };
    }

    // async fn get_pool(&self) -> anyhow::Result<&Pool<Postgres>> {
    //     match self
    //         .pool
    //         .get_or_try_init(|| async {
    //             let user = self.settings.user.clone();
    //             let pw = self.settings.pw.clone();
    //             let host = self.settings.host.clone();
    //             let port = self.settings.port.clone();
    //             let db = self.settings.name.clone();

    //             let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

    //             PgPoolOptions::new().max_connections(5).connect(&uri).await
    //         })
    //         .await
    //     {
    //         Err(e) => {
    //             return Err(anyhow::anyhow!(PostgresError::ConnectionError(e)));
    //         }
    //         Ok(pool) => {
    //             return Ok(pool);
    //         }
    //     }
    // }

    async fn fetch_tables(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(
            "
        select \
            table_name \
        from information_schema.tables \
        where \
          table_type = 'BASE TABLE' \
          and table_schema = $1",
            args,
        );
        let query = builder.build();

        // let pool = self.pool.await?;
        let results = query.fetch_all(&self.pool).await?;
        let tables: Vec<String> = results
            .into_iter()
            .map(|r| r.get::<String, _>("table_name"))
            .collect();
        return Ok(tables);
    }
}

#[async_trait]
impl Node for SchemaTablesNode {
    fn name(&self) -> &str {
        return "Tables";
    }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     debug!("SchemaTableNode::children");

    //     return None;
    // }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_tables().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
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
