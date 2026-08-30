use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::OnceCell;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{PgPool, Pool, Postgres, Row};

use gtk::gio;

use silo_plugin::node::{DataSourceNode, Node};

use crate::{PostgresError, nodes::ConnectionSettings};

#[derive(Debug, Clone)]
pub struct TableNode {
    table_name: String,

    settings: Arc<ConnectionSettings>,
    pool: OnceCell<Pool<Postgres>>,
}

impl TableNode {
    pub fn new(table_name: &str, settings: Arc<ConnectionSettings>) -> Self {
        return Self {
            table_name: table_name.to_string(),
            settings: settings,
            pool: OnceCell::new(),
        };
    }
}

#[async_trait]
impl Node for TableNode {
    fn name(&self) -> &str {
        return self.table_name.as_str();
    }

    fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        return None;
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Ok(None);
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        return None;
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }
}
