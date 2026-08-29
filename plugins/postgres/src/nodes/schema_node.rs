use tracing::debug;

use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;

use silo_plugin::node::{DataSourceNode, Node};

use crate::nodes::ConnectionSettings;
use crate::nodes::schema_functions_node::SchemaFunctionsNode;
use crate::nodes::schema_procedures_node::SchemaProceduresNode;
use crate::nodes::schema_tables_node::SchemaTablesNode;

#[derive(Debug, Clone)]
pub struct SchemaNode {
    pub name: String,

    pub settings: Arc<ConnectionSettings>,
}

impl SchemaNode {
    pub fn new(name: &str, settings: Arc<ConnectionSettings>) -> Self {
        return Self {
            name: name.to_string(),
            settings,
        };
    }
}

#[async_trait]
impl Node for SchemaNode {
    fn name(&self) -> &str {
        return &self.name.as_str();
    }

    // fn clone_box(&self) -> Box<dyn Node> {
    //     return Box::new(self.clone());
    // }

    fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        debug!("SchemaNode::children");

        let mut nodes: Vec<Arc<dyn Node>> = vec![];

        let boxed: Arc<dyn Node> = Arc::new(SchemaTablesNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        let boxed: Arc<dyn Node> = Arc::new(SchemaProceduresNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        let boxed: Arc<dyn Node> = Arc::new(SchemaFunctionsNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        return Some(nodes);
    }

    async fn children_async(&self) -> Result<Option<Vec<Arc<dyn Node>>>, &'static str> {
        return Err("//todo not implemented");
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return None;
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}
