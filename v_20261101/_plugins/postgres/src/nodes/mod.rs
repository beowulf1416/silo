pub mod column_node;
pub mod data_source_node;
pub mod function_node;
pub mod procedure_node;
pub mod schema_functions_node;
pub mod schema_node;
pub mod schema_procedures_node;
pub mod schema_sequences_node;
pub mod schema_tables_node;
pub mod schema_views_node;
pub mod sequence_node;
pub mod table_columns_node;
pub mod table_indexes_node;
pub mod table_node;
pub mod view_node;

#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    pub name: String,
    pub db: String,
    pub host: String,
    pub port: u32,
    pub user: String,
    pub pw: String,
}
