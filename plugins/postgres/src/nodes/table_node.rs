use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::error;

use sqlx::{Arguments, Row};

use gtk::gio;

use silo_plugin::node::{DataSourceNode, Node};

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct TableNode {
    schema_name: String,
    table_name: String,

    // settings: Arc<ConnectionSettings>,
    // pool: OnceCell<Pool<Postgres>>,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TableNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, schema_name: &str, table_name: &str) -> Self {
        return Self {
            schema_name: schema_name.to_string(),
            table_name: table_name.to_string(),
            pool,
        };
    }

    pub async fn fetch_columns(&self) -> anyhow::Result<Vec<String>> {
        let mut args = sqlx::postgres::PgArguments::default();
        args.add(&self.schema_name);
        args.add(&self.table_name);

        let mut builder = sqlx::QueryBuilder::with_arguments(
            "
        select \
            column_name, \
            data_type, \
            column_default, \
            is_nullable \
        from information_schema.columns \
        where \
          table_schema = $1 \
          and table_name = $2",
            args,
        );
        let query = builder.build();

        // let pool = self.pool.await?;
        let results = query.fetch_all(&self.pool).await?;
        let columns: Vec<String> = results
            .into_iter()
            .map(|r| r.get::<String, _>("column_name"))
            .collect();
        return Ok(columns);
    }
}

#[async_trait]
impl Node for TableNode {
    fn name(&self) -> &str {
        return self.table_name.as_str();
    }

    // fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
    //     return None;
    // }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        match self.fetch_columns().await {
            Err(e) => {
                error!("unable to fetch columns {}", e);
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
        return None;
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }
}
