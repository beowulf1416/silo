use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::data_sources_view::node::Node;

#[derive(Debug, Clone)]
pub struct SchemaTablesNode {
    // pub name: String,
}

impl SchemaTablesNode {
    // pub fn new(name: &str) -> Self {
    //     return Self {
    //         name: name.to_string(),
    //     };
    // }
}

impl Node for SchemaTablesNode {
    fn name(&self) -> &str {
        return "Tables";
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<gio::ListStore> {
        debug!("SchemaTableNode::children");

        return None;
    }
}
