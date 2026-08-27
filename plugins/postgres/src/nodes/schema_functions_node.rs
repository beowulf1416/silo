use tracing::debug;

use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::node::Node;

use crate::nodes::ConnectionSettings;

#[derive(Debug, Clone)]
pub struct SchemaFunctionsNode {
    pub settings: Arc<ConnectionSettings>,
}

impl SchemaFunctionsNode {
    pub fn new(settings: Arc<ConnectionSettings>) -> Self {
        return Self { settings };
    }
}

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

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }
}
