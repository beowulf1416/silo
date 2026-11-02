use anyhow::anyhow;
use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use silo_plugin::node::{DataSourceNode, Node};
use tracing::error;

use sqlx::{Arguments, Row};
use std::sync::Arc;

use crate::{PostgresError, nodes::procedure_node::ProcedureNode};

#[derive(Debug, Clone)]
pub struct SchemaProceduresNode {
    pool: sqlx::Pool<sqlx::Postgres>,
    schema_name: String,
}

impl SchemaProceduresNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            pool,
            schema_name: schema_name.clone(),
        };
    }

    pub async fn fetch_procedures(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        let _ = args.add(&self.schema_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(
            "select \
              p.proname proc_name
            from pg_proc p \
              join pg_namespace n \
                on p.pronamespace = n.oid \
            where \
              n.nspname = $1 \
              and p.prokind = 'p'",
            args,
        );
        let query = builder.build();

        let results = query.fetch_all(&self.pool).await?;
        let procs: Vec<String> = results
            .into_iter()
            .map(|r| r.get::<String, _>("proc_name"))
            .collect();
        return Ok(procs);
    }
}

#[async_trait]
impl Node for SchemaProceduresNode {
    fn name(&self) -> &str {
        return "Procedures";
    }

    // fn clone_box(&self) -> Arc<dyn Node> {
    //     return Arc::new(self.clone());
    // }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     debug!("SchemaProceduresNode::children");

    //     return None;
    // }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_procedures().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
            }
            Ok(procs) => {
                // let schema_name = self.schema_name.clone();
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for proc in procs {
                    let boxed: Arc<dyn Node> = Arc::new(ProcedureNode::new(
                        self.pool.clone(),
                        &self.schema_name,
                        &proc,
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
    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}
