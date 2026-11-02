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
pub struct ProcedureNode {
    schema_name: String,
    procedure_name: String,

    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ProcedureNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, proc_name: &str) -> Self {
        return Self {
            schema_name: schema_name.to_string(),
            procedure_name: proc_name.to_string(),
            pool,
        };
    }
}

#[async_trait]
impl Node for ProcedureNode {
    fn name(&self) -> &str {
        return self.procedure_name.as_str();
    }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     return None;
    // }

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
