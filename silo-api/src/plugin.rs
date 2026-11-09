use std::sync::Arc;

use crate::{Silo, SiloError, data_sources::DataSource, node::Node};

#[async_trait::async_trait]
pub trait Plugin: std::fmt::Debug + Send + Sync {
    fn name(&self) -> String;
    fn set_silo(&self, silo: Arc<dyn Silo>);
    fn register_source(&self, source: &DataSource);
    async fn get_child_nodes(&self, node: &dyn Node) -> Vec<Arc<dyn Node>>;
}

#[async_trait::async_trait]
pub trait PluginManager: std::fmt::Debug + Send + Sync {
    fn register(&mut self, plugin: Arc<dyn Plugin>) -> Result<(), SiloError>;
    // fn initialize_plugins(&self, silo: Arc<dyn Silo>, sources: &Vec<DataSource>);
    fn plugins(&self) -> Vec<Arc<dyn Plugin>>;
    fn get_plugin(&self, plugin_name: &str) -> Option<Arc<dyn Plugin>>;
    // async fn get_child_nodes(&self, node: &dyn Node) -> Option<Vec<Arc<dyn Node>>>;
    async fn process_query(&self, plugin: &str, query: &str) -> Result<(), SiloError>;
}
