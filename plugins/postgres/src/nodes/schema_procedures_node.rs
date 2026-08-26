use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::components::data_sources_view::node::Node;
use silo_plugin::node::Node;

#[derive(Debug, Clone)]
pub struct SchemaProceduresNode {}

impl Node for SchemaProceduresNode {
    fn name(&self) -> &str {
        return "Procedures";
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<gio::ListStore> {
        debug!("SchemaProceduresNode::children");

        return None;
    }
}
