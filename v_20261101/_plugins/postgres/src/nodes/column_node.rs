use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::error;

use sqlx::{Arguments, Row};

use gtk::gio;

use crate::nodes;
use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct ColumnNode {
    schema_name: String,
    table_name: String,
    column_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ColumnNode {
    pub fn new(
        pool: sqlx::Pool<sqlx::Postgres>,
        schema_name: &str,
        table_name: &str,
        column_name: &str,
    ) -> Self {
        return Self {
            schema_name: schema_name.to_string(),
            table_name: table_name.to_string(),
            column_name: column_name.to_string(),
            pool,
        };
    }
}

#[async_trait]
impl Node for ColumnNode {
    fn name(&self) -> &str {
        return self.column_name.as_str();
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Err(anyhow!("//todo not implemented ColumnNode"));
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        return None;
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }
}
