use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;
use crate::nodes::sequence_node::SequenceNode;

static sql: &str = "
select \
    ns.nspname AS schema_name, \
    cls.relname AS sequence_name, \
    pg_get_userbyid(cls.relowner) AS owner \
from pg_class cls \
    join pg_namespace ns ON ns.oid = cls.relnamespace \
where
    cls.relkind = 'S' -- 'S' filters specifically for sequences \
    and ns.nspname = $1
";

#[derive(Debug, Clone)]
pub struct SchemaSequencesNode {
    schema_name: String,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SchemaSequencesNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            schema_name: schema_name.clone(),
            pool: pool,
        };
    }

    async fn fetch_sequences(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(sql, args);
        let query = builder.build();

        let results = query.fetch_all(&self.pool).await?;
        let sequences: Vec<String> = results
            .into_iter()
            .map(|r| r.get::<String, _>("sequence_name"))
            .collect();
        return Ok(sequences);
    }
}

#[async_trait]
impl Node for SchemaSequencesNode {
    fn name(&self) -> &str {
        return "Sequences";
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_sequences().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
            }
            Ok(sequences) => {
                // let schema_name = self.schema_name.clone();
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for sequence in sequences {
                    let boxed: Arc<dyn Node> = Arc::new(SequenceNode::new(
                        self.pool.clone(),
                        &self.schema_name,
                        &sequence,
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
