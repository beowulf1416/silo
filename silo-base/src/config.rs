use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataSourceConfig {
    pub name: String,
    pub data_source_name: String,
    pub host: String,
    pub port: i32,
    pub user: String,
}
