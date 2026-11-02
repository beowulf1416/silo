use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::error;

use sqlx::{Arguments, Row};

use gtk::gio;

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct SequenceNode {
    schema_name: String,
    sequence_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SequenceNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, table_name: &str) -> Self {
        return Self {
            schema_name: schema_name.to_string(),
            sequence_name: table_name.to_string(),
            pool,
        };
    }
}

#[async_trait]
impl Node for SequenceNode {
    fn name(&self) -> &str {
        return self.sequence_name.as_str();
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Err(anyhow!("//todo sequence_node"));
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        return None;
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }
}
