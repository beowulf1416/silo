use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::debug;

// use crate::components::data_sources_view::node::Node;
use crate::nodes::ConnectionSettings;
use silo_plugin::node::Node;

#[derive(Debug, Clone)]
pub struct SchemaProceduresNode {
    pub settings: Arc<ConnectionSettings>,
}

impl SchemaProceduresNode {
    pub fn new(settings: Arc<ConnectionSettings>) -> Self {
        return Self { settings };
    }
}

impl Node for SchemaProceduresNode {
    fn name(&self) -> &str {
        return "Procedures";
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<Vec<Box<dyn Node>>> {
        debug!("SchemaProceduresNode::children");

        return None;
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }
}
