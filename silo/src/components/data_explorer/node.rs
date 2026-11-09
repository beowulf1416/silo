use std::sync::Arc;
use std::vec::Vec;
use tracing::{debug, error, info};

use silo_api::{
    Silo,
    node::{Node, NodeType},
    plugin::Plugin,
};

// use crate::components::app_window::AppWindow;

#[derive(Debug)]
pub struct NodeImpl {
    pub name: String,
    pub node_type: NodeType,
    pub children: Option<Vec<Arc<dyn Node>>>,
    pub plugin: Option<Arc<dyn Plugin>>,
    pub dsn: String,
    pub connection: String,
    // pub silo: Option<Arc<dyn Silo>>,
}

impl NodeImpl {
    pub fn new(
        name: &String,
        node_type: NodeType,
        plugin: Option<Arc<dyn Plugin>>,
        dsn: &String,
        connection: &String,
    ) -> Self {
        info!("NodeImpl::new()");

        return Self {
            name: name.clone(),
            dsn: dsn.clone(),
            node_type: node_type,
            children: None,
            plugin: plugin.clone(),
            connection: connection.clone(),
            // silo: None,
        };
    }
}

#[async_trait::async_trait]
impl Node for NodeImpl {
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn node_type(&self) -> NodeType {
        return self.node_type.clone();
    }

    fn plugin(&self) -> Option<Arc<dyn Plugin>> {
        return self.plugin.clone();
    }

    fn data_source_name(&self) -> String {
        return self.dsn.clone();
    }

    fn set_silo(&mut self, silo: Arc<dyn Silo>) {
        info!("NodeImpl::set_silo()");
        // self.silo = Some(silo);
    }

    fn silo(&self) -> Option<Arc<dyn Silo>> {
        info!("NodeImpl::silo()");
        // return self.silo.clone();
        return None;
    }

    async fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        info!("NodeImpl::children()");

        // let parent_node: Arc<dyn Node> = Arc::new(self);
        // let nodes = silo.as_ref().plugin_manager().get_child_nodes(self);

        // let pm = self
        //     .silo
        //     .clone()
        //     .expect("expected silo object")
        //     .as_ref()
        //     .plugin_manager();
        // let nodes = pm.get_child_nodes(self).await;
        // return nodes;
        //

        if let Some(plugin) = self.plugin.clone() {
            let result = plugin.get_child_nodes(self).await;
            debug!("NodeImpl::children() result {:?}", result);
            // match plugin.get_child_nodes(self).await {
            //     Err(e) => {
            //         error!("NodeImpl::children() unable to obtain nodes {:?}", e);
            //         return None;
            //     }
            //     Ok(result) => {
            //         debug!("result {:?}", result);
            //         return None;
            //     }
            // }
        } else {
            error!("NodeImpl::children() unable to obtain plugin for node");
        }

        return None;
    }
}

#[derive(Debug)]
pub struct LoadingNode {}

#[async_trait::async_trait]
impl Node for LoadingNode {
    fn name(&self) -> String {
        return String::from("Loading...");
    }

    fn node_type(&self) -> NodeType {
        return NodeType::Loading;
    }

    fn plugin(&self) -> Option<Arc<dyn Plugin>> {
        return None;
    }

    fn data_source_name(&self) -> String {
        return String::from("");
    }

    fn silo(&self) -> Option<Arc<dyn Silo>> {
        return None;
    }

    fn set_silo(&mut self, silo: Arc<dyn Silo>) {
        // do nothing
    }

    async fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        return None;
    }
}
