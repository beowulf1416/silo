use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::{debug, error};

use sqlx::{Arguments, Row};

use silo_plugin::node::{DataSourceNode, Node};

use crate::nodes;

static sql: &str = "
    select \
        column_name, \
        data_type, \
        column_default, \
        is_nullable \
    from information_schema.columns \
    where \
      table_schema = $1 \
      and table_name = $2";

#[derive(Debug, Clone)]
pub struct TableColumnsNode {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub schema_name: String,
    pub table_name: String,
}

impl TableColumnsNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, table_name: &str) -> Self {
        return Self {
            pool,
            schema_name: schema_name.to_string(),
            table_name: table_name.to_string(),
        };
    }

    pub async fn fetch_columns(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);
        let _ = args.add(&self.table_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(sql, args);
        let query = builder.build();

        let results = query.fetch_all(&self.pool).await?;
        let columns: Vec<String> = results
            .into_iter()
            .map(|r| r.get::<String, _>("column_name"))
            .collect();
        return Ok(columns);
    }
}

#[async_trait]
impl Node for TableColumnsNode {
    fn name(&self) -> &str {
        return &"Columns";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_columns().await {
            Err(e) => {
                error!("unable to fetch columns {}", e);
                return Err(anyhow!("unable to fetch columns"));
            }
            Ok(columns) => {
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for column in columns {
                    let boxed: Arc<dyn Node> = Arc::new(nodes::column_node::ColumnNode::new(
                        self.pool.clone(),
                        &self.schema_name,
                        &self.table_name,
                        &column,
                    ));
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
