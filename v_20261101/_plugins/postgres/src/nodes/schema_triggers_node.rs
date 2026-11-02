use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;
use crate::nodes::trigger_node::TriggerNode;

static sql: &str = "
select \
    evt.evtname AS trigger_name, \
    evt.evtevent AS event_type, -- ddl_command_start, ddl_command_end, sql_drop, table_rewrite \
    proc.proname AS function_name, \
    pg_get_userbyid(evt.evtowner) AS owner, \
    CASE evt.evtenabled \
        WHEN 'O' THEN 'Enabled' \
        WHEN 'D' THEN 'Disabled' \
        WHEN 'R' THEN 'Replica' \
        WHEN 'A' THEN 'Always' \
    END AS status, \
    array_to_string(evt.evttags, ', ') AS command_tags -- Filtered DDL tags (if any) \
FROM pg_event_trigger evt
JOIN pg_proc proc ON evt.evtfoid = proc.oid \
";

#[derive(Debug, Clone)]
pub struct SchemaTriggersNode {
    schema_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SchemaTriggersNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            pool: pool,
        };
    }

    async fn fetch_triggers(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(sql, args);
        let query = builder.build();

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
}
