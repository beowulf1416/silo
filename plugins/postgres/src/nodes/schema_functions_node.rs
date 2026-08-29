use tracing::debug;

use async_trait::async_trait;
use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::nodes::ConnectionSettings;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct SchemaFunctionsNode {
    pub settings: Arc<ConnectionSettings>,
}

impl SchemaFunctionsNode {
    pub fn new(settings: Arc<ConnectionSettings>) -> Self {
        return Self { settings };
    }
}

#[async_trait]
impl Node for SchemaFunctionsNode {
    fn name(&self) -> &str {
        return "Functions";
    }

    // fn clone_box(&self) -> Box<dyn Node> {
    //     return Box::new(self.clone());
    // }

    fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        debug!("SchemaFunctionsNode::children");

        return None;
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
