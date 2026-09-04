use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use sqlx::{Arguments, Row};
use std::sync::Arc;
use tracing::{debug, error};

// use crate::nodes::ConnectionSettings;
use crate::PostgresError;
use crate::nodes::function_node::FunctionNode;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct SchemaFunctionsNode {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    // pub settings: Arc<ConnectionSettings>,
    pub schema_name: String,
}

impl SchemaFunctionsNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &String) -> Self {
        return Self {
            pool,
            schema_name: schema_name.clone(),
        };
    }

    pub async fn fetch_functions(&self) -> anyhow::Result<Vec<String>> {
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
                  and p.prokind = 'f'",
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
impl Node for SchemaFunctionsNode {
    fn name(&self) -> &str {
        return "Functions";
    }

    // fn clone_box(&self) -> Box<dyn Node> {
    //     return Box::new(self.clone());
    // }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     debug!("SchemaFunctionsNode::children");

    //     return None;
    // }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_functions().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow::anyhow!(PostgresError::SchemaError));
            }
            Ok(functions) => {
                // let schema_name = self.schema_name.clone();
                let mut result: Vec<Arc<dyn Node>> = vec![];
                for func in functions {
                    let boxed: Arc<dyn Node> = Arc::new(FunctionNode::new(
                        self.pool.clone(),
                        &self.schema_name,
                        &func,
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
