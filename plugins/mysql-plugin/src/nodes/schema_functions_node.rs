use tracing::debug;

use async_trait::async_trait;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;

// use crate::components::data_sources_view::node::Node;
use crate::nodes::ConnectionSettings;
use silo_plugin::node::Node;

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

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<Vec<Box<dyn Node>>> {
        debug!("SchemaFunctionsNode::children");

        return None;
    }

    async fn children_async(&self) -> Result<Option<Vec<Box<dyn Node>>>, &'static str> {
        return Err("//todo not implemented");
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}
