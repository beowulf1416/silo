use anyhow::anyhow;
use async_trait::async_trait;
use gtk::gio;
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use crate::PostgresError;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct FunctionNode {
    schema_name: String,
    function_name: String,

    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FunctionNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, function_name: &str) -> Self {
        return Self {
            schema_name: schema_name.to_string(),
            function_name: function_name.to_string(),
            pool,
        };
    }
}

#[async_trait]
impl Node for FunctionNode {
    fn name(&self) -> &str {
        return self.function_name.as_str();
    }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     return None;
    // }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Err(anyhow::anyhow!("//todo not implemented"));
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        return None;
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }
}
