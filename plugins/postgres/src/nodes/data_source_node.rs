use tracing::{debug, error};

use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};

use std::iter::Iterator;

use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;

use sqlx::{
    Column, TypeInfo,
    postgres::{PgColumn, PgPoolOptions, PgRow},
    query::Query,
};
use sqlx::{PgPool, Pool, Postgres, Row};
use tokio::sync::OnceCell;

use crate::get_runtime;
use crate::nodes::schema_node::SchemaNode;
use crate::{PostgresError, nodes::ConnectionSettings};
use silo_plugin::node::{DataSourceNode, Node, QueryColumn, QueryResult};

#[derive(Debug, Clone)]
pub struct PostgresDataSourceNode {
    name: String,

    // settings: Arc<ConnectionSettings>,
    // pool: OnceCell<Pool<Postgres>>,
    pool: sqlx::postgres::PgPool,
}

impl PostgresDataSourceNode {
    pub fn new(name: &str, pool: &sqlx::postgres::PgPool) -> Self {
        return Self {
            name: name.to_string(),
            // settings: Arc::new(settings),
            // pool: OnceCell::new(),
            pool: pool.clone(),
        };
    }

    // async fn get_pool(&self) -> anyhow::Result<&Pool<Postgres>> {
    //     match self
    //         .pool
    //         .get_or_try_init(|| async {
    //             let user = self.settings.user.clone();
    //             let pw = self.settings.pw.clone();
    //             let host = self.settings.host.clone();
    //             let port = self.settings.port.clone();
    //             let db = self.settings.db.clone();

    //             let uri = format!("postgres://{user}:{pw}@{host}:{port}/{db}");

    //             match PgPoolOptions::new().max_connections(5).connect(&uri).await {
    //                 Err(e) => {
    //                     error!("unable to connect to database: {}", e);
    //                     return Err(anyhow::anyhow!(PostgresError::ConnectionError(e)));
    //                 }
    //                 Ok(pool) => {
    //                     return Ok(pool);
    //                 }
    //             }
    //         })
    //         .await
    //     {
    //         Err(e) => {
    //             // return Err(anyhow::anyhow!(PostgresError::ConnectionError(e)));
    //             return Err(e);
    //         }
    //         Ok(pool) => {
    //             return Ok(pool);
    //         }
    //     }
    // }

    async fn fetch_schemas_async(&self) -> anyhow::Result<Vec<String>> {
        debug!("fetch_schemas_async");

        let pool = self.pool.clone();
        let sql = "select schema_name from information_schema.schemata";

        let handle = get_runtime().spawn(async move {
            return sqlx::query(sql).fetch_all(&pool).await;
        });

        match handle.await {
            Err(e) => {
                // JoinError
                error!("unable to fetch schemas: {}", e);
                return Err(anyhow::anyhow!(e));
            }
            Ok(result) => match result {
                Err(e) => {
                    error!("unable to fetch schemas: {}", e);
                    return Err(anyhow::anyhow!(e));
                }
                Ok(rows) => {
                    let schemas: Vec<String> = rows
                        .into_iter()
                        .map(|r| r.get::<String, _>("schema_name"))
                        .collect();
                    return Ok(schemas);
                }
            },
        }

        // match sqlx::query(sql).fetch_all(pool).await {
        //     Err(e) => {
        //         error!("unable to fetch schemas: {}", e);
        //         return Err(anyhow::anyhow!(e));
        //     }
        //     Ok(results) => {
        //         let schemas: Vec<String> = results
        //             .into_iter()
        //             .map(|r| r.get::<String, _>("schema_name"))
        //             .collect();
        //         return Ok(schemas);
        //     }
        // }

        // match self.pool.await {
        //     Err(e) => {
        //         error!("unable to get pool: {}", e);
        //         return Err(anyhow::anyhow!(e));
        //     }
        //     Ok(pool) => {
        //         let sql = "select schema_name from information_schema.schemata";
        //         match sqlx::query(sql).fetch_all(pool).await {
        //             Err(e) => {
        //                 error!("unable to fetch schemas: {}", e);
        //                 return Err(anyhow::anyhow!(e));
        //             }
        //             Ok(results) => {
        //                 let schemas: Vec<String> = results
        //                     .into_iter()
        //                     .map(|r| r.get::<String, _>("schema_name"))
        //                     .collect();
        //                 return Ok(schemas);
        //             }
        //         }
        //     }
        // }
    }

    fn decode(&self, r: &PgRow, c: &PgColumn, i: usize) -> String {
        /*
        SELECT typname AS type_name
        FROM pg_type
        WHERE typnamespace = 'pg_catalog'::regnamespace
        ORDER BY typname;
        */

        let value = match c.type_info().name() {
            // _ => {
            //     debug!("unknown type: {}", c.type_info().name());
            //     "[UNKNOWN]".to_string()
            // }
            "BOOL" => r.get::<bool, _>(i).to_string(),
            "CHAR" => r.get::<String, _>(i).to_string(),
            "VARCHAR" => {
                if let Some(value) = r.get::<Option<String>, _>(i) {
                    return value;
                }
                return "[NULL]".to_string();
            }
            "NAME" => {
                if let Some(value) = r.get::<Option<String>, _>(i) {
                    return value;
                }
                return "[NULL]".to_string();
            }
            _ => {
                debug!("unknown type: {}", c.type_info().name());
                "[UNKNOWN]".to_string()
            }
        };

        return value;
    }
}

#[async_trait]
impl Node for PostgresDataSourceNode {
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        debug!("PostgresDataSourceNode::children");

        match self.fetch_schemas_async().await {
            Err(e) => {
                error!("unable to fetch children async {}", e);
                return Err(anyhow!(PostgresError::SchemaError));
            }
            Ok(schemas) => {
                // let pool = self.get_pool().await?;
                let pool = self.pool.clone();

                let mut result: Vec<Arc<dyn Node>> = vec![];
                for schema in schemas {
                    let boxed: Arc<dyn Node> =
                        Arc::new(SchemaNode::new(pool.clone(), schema.as_str()));
                    result.push(boxed);
                }
                return Ok(Some(result));
            }
        }
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Edit"), Some("win.data-source-edit::postgres"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);

        let item = gio::MenuItem::new(
            Some("New Query"),
            Some("win.data-source-new-query::postgres"),
        );
        let section = gio::Menu::new();
        section.append_item(&item);

        menu.append_section(None, &section);

        let item = gio::MenuItem::new(Some("Remove"), Some("win.data-source-remove::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return Some(menu);
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return Some(Arc::new(self.clone()));
    }
}

#[async_trait]
impl DataSourceNode for PostgresDataSourceNode {
    async fn query(&self, sql: &str) -> anyhow::Result<QueryResult> {
        debug!("PostgresDataSourceNode::query {}", sql);

        let sql = String::from(sql);
        let pool = self.pool.clone();

        let handle = get_runtime().spawn(async move {
            let mut builder: sqlx::QueryBuilder<Postgres> = sqlx::QueryBuilder::new(sql);
            let query = builder.build();

            return query.fetch_all(&pool).await;
        });

        match handle.await {
            Err(e) => {
                error!("unable to execute query: {}", e);
                return Err(anyhow::anyhow!(e));
            }
            Ok(result) => match result {
                Err(e) => {
                    error!("unable to execute query: {}", e);
                    return Err(anyhow::anyhow!(e));
                }
                Ok(rows) => {
                    if rows.is_empty() {
                        return Ok(QueryResult {
                            columns: vec![],
                            rows: vec![],
                        });
                    }

                    let mut columns: Vec<QueryColumn> = vec![];
                    if let Some(first_row) = rows.get(0) {
                        columns = first_row
                            .columns()
                            .iter()
                            .map(|c| QueryColumn {
                                name: c.name().to_string(),
                                data_type: c.type_info().name().to_string(),
                            })
                            .collect();
                    }
                    debug!("columns: {:?}", columns);

                    let rows: Vec<Vec<String>> = rows
                        .iter()
                        // .map(|r| r.iter().map(|c| c.to_string()).collect())
                        .map(|r| {
                            r.columns()
                                .iter()
                                .enumerate()
                                .map(|(i, c)| self.decode(r, c, i))
                                .collect()
                        })
                        .collect();

                    return Ok(QueryResult {
                        columns: columns,
                        rows: rows,
                    });
                }
            },
        }
    }

    fn get_configuration(&self) -> serde_json::Value {
        return serde_json::json!({
            "name": self.name.clone(),
            // "host": self.settings.host.clone(),
            // "port": self.settings.port.clone(),
            // "user": self.settings.user.clone(),
            // "pw": self.settings.pw.clone(),
        });
    }
}
