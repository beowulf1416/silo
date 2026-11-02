pub mod data_source_node;
pub mod schema_functions_node;
pub mod schema_procedures_node;
pub mod schema_tables_node;
pub mod table_node;

#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    pub name: String,
    pub db: String,
    pub host: String,
    pub port: u32,
    pub user: String,
    pub pw: String,
}
