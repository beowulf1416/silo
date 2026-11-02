use std::sync::Arc;

use crate::Silo;

#[derive(Debug, Clone)]
pub enum NodeType {
    Loading,
    DataSource,
    Schema,
    Table,
    View,
    Column,
    Index,
    Constraint,
}

#[async_trait::async_trait]
pub trait Node: std::fmt::Debug + Send + Sync {
    fn name(&self) -> String;
    fn data_source_name(&self) -> String;
    fn node_type(&self) -> NodeType;
    fn plugin(&self) -> String;
    fn set_silo(&mut self, silo: Arc<dyn Silo>);
    fn silo(&self) -> Option<Arc<dyn Silo>>;
    async fn children(&self) -> Option<Vec<Arc<dyn Node>>>;
}
