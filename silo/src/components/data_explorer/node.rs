use std::sync::Arc;
use std::vec::Vec;

use silo_api::{
    Silo,
    node::{Node, NodeType},
};

#[derive(Debug)]
pub struct NodeImpl {
    pub name: String,
    pub node_type: NodeType,
    pub children: Option<Vec<Arc<dyn Node>>>,
    pub plugin: String,
    pub dsn: String,
    pub connection: String,
    pub silo: Option<Arc<dyn Silo>>,
}

impl NodeImpl {
    pub fn new(
        name: &String,
        node_type: NodeType,
        plugin: &String,
        dsn: &String,
        connection: &String,
    ) -> Self {
        return Self {
            name: name.clone(),
            dsn: dsn.clone(),
            node_type: node_type,
            children: None,
            plugin: plugin.clone(),
            connection: connection.clone(),
            silo: None,
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

    fn plugin(&self) -> String {
        return self.plugin.clone();
    }

    fn data_source_name(&self) -> String {
        return self.dsn.clone();
    }

    fn set_silo(&mut self, silo: Arc<dyn Silo>) {
        self.silo = Some(silo);
    }

    fn silo(&self) -> Option<Arc<dyn Silo>> {
        return self.silo.clone();
    }

    async fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        // let parent_node: Arc<dyn Node> = Arc::new(self);
        // let nodes = silo.as_ref().plugin_manager().get_child_nodes(self);
        let pm = self
            .silo
            .clone()
            .expect("expected silo object")
            .as_ref()
            .plugin_manager();
        let nodes = pm.get_child_nodes(self).await;
        return nodes;
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

    fn plugin(&self) -> String {
        return String::from("default");
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
