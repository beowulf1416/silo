use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub plugin: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pw: String,
    pub db: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSources {
    pub sources: RwLock<std::collections::HashMap<String, DataSource>>,
}

impl Default for DataSources {
    fn default() -> Self {
        Self {
            sources: RwLock::new(std::collections::HashMap::new()),
        }
    }
}
