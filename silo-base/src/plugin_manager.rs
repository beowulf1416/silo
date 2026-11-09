use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

use silo_api::{
    Silo, SiloError,
    data_sources::DataSource,
    node::{Node, NodeType},
    plugin::{Plugin, PluginManager},
};

#[derive(Debug)]
pub struct PluginManagerImpl {
    plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl PluginManagerImpl {
    pub fn new() -> Self {
        return Self {
            plugins: HashMap::new(),
        };
    }
}

#[async_trait::async_trait]
impl PluginManager for PluginManagerImpl {
    fn register(&mut self, plugin: Arc<dyn Plugin>) -> Result<(), SiloError> {
        info!("PluginManagerImpl::register");
        let plugin_name = plugin.name();
        self.plugins.insert(plugin_name, plugin);
        return Ok(());
    }

    fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
        info!("PluginManagerImpl::plugins()");

        let plugins: Vec<Arc<dyn Plugin>> = self.plugins.clone().into_values().collect();
        return plugins;
    }

    //     fn initialize_plugins(&self, silo: Arc<dyn Silo>, sources: &Vec<DataSource>) {
    //         info!("PluginManagerImpl::initialize_plugins()");
    //
    //         for s in sources {
    //             let plugin = s.plugin.as_str();
    //
    //             if let Some(plugin) = self.plugins.get(plugin) {
    //                 plugin.initialize(silo.clone(), &s);
    //             } else {
    //                 error!("plugin registered but not found: {}", plugin);
    //             }
    //         }
    //     }

    fn get_plugin(&self, plugin_name: &str) -> Option<Arc<dyn Plugin>> {
        let result = self.plugins.get(plugin_name).map(Arc::clone);
        return result;
        // let plugin = result.as_ref();
        // return Some(Arc::new(*plugin));
    }

    //     async fn get_child_nodes(&self, node: &dyn Node) -> Option<Vec<Arc<dyn Node>>> {
    //         info!("PluginManagerImpl::get_child_nodes()");
    //
    //         let plugin = node.plugin();
    //         // let plugin_name = plugin.as_str();
    //         let name = node.name();
    //
    //         if let Some(plugin) = node.plugin() {
    //
    //         } else {
    //             error!("")
    //         }
    //
    //         if let Some(plugin) = self.plugins.get(plugin_name) {
    //             if let Some(silo) = node.silo() {
    //                 if let Some(plugin_impl) = silo
    //                     .as_ref()
    //                     .plugin_manager()
    //                     .as_ref()
    //                     .get_plugin(node.plugin().as_str())
    //                 {
    //                     let child_nodes = plugin_impl.as_ref().get_child_nodes(node).await;
    //                     return Some(child_nodes);
    //                 } else {
    //                     error!("unable to obtain plugin");
    //                 }
    //             } else {
    //                 error!("silo is not set");
    //             }
    //
    //             info!("fetch child nodes");
    //         } else {
    //             error!("plugin not found");
    //         }
    //
    //         return None;
    //     }

    async fn process_query(&self, plugin: &str, query: &str) -> Result<(), SiloError> {
        info!("PluginManagerImpl::process_query");
        todo!();
    }
}
