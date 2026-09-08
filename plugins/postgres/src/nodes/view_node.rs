use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct ViewNode {
    schema_name: String,
    view_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ViewNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String, view_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            view_name: view_name.clone(),
            pool: pool,
        };
    }

    pub async fn fetch_columns(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);
        let _ = args.add(&self.view_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(
            "
            select \
                column_name, \
                data_type, \
                column_default, \
                is_nullable \
            from information_schema.columns \
            where \
              table_schema = $1 \
              and table_name = $2",
            args,
        );
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
impl Node for ViewNode {
    fn name(&self) -> &str {
        return "Views";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_columns().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
            }
            Ok(views) => {
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
