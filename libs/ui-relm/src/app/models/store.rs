use crate::app::models::schema::Schema;

#[derive(Debug, Clone)]
pub struct Store {
    pub name: String,
    pub object_types: Vec<StoreObjectType>,
}

#[derive(Debug, Clone)]
pub struct StoreObjectType {
    pub name: String,
    pub object_types: Vec<StoreObjectType>,
}
