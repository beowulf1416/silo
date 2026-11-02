use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub data_source_name: String,
    pub plugin: String,
    pub host: String,
    pub port: u32,
    pub user: String,
}
