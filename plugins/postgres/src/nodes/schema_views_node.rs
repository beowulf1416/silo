use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;
use crate::nodes::view_node::ViewNode;

#[derive(Debug, Clone)]
pub struct SchemaViewsNode {
    schema_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SchemaViewsNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            // settings: settings,
            pool: pool,
        };
    }

    async fn fetch_views(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(
            "
        select \
            table_name \
        from information_schema.views \
        where \
          table_schema = $1",
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
impl Node for SchemaViewsNode {
    fn name(&self) -> &str {
        return "Views";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_views().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
            }
            Ok(views) => {
                // let schema_name = self.schema_name.clone();
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for view in views {
                    let boxed: Arc<dyn Node> =
                        Arc::new(ViewNode::new(self.pool.clone(), &self.schema_name, &view));
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
}
