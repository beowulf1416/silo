use tracing::debug;

use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;

use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct TableIndexesNode {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub schema_name: String,
    pub table_name: String,
}

impl TableIndexesNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, table_name: &str) -> Self {
        return Self {
            pool,
            schema_name: schema_name.to_string(),
            table_name: table_name.to_string(),
        };
    }
}

#[async_trait]
impl Node for TableIndexesNode {
    fn name(&self) -> &str {
        return &"Indexes";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Err(anyhow::anyhow!("//todo not implemented TableIndexesNode"));
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
