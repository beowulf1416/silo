use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::data_sources_view::node::Node;

#[derive(Debug, Clone)]
pub struct SchemaFunctionsNode {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl SchemaFunctionsNode {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        return Self { pool };
    }
}

impl Node for SchemaFunctionsNode {
    fn name(&self) -> &str {
        return "Functions";
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<gio::ListStore> {
        debug!("SchemaFunctionsNode::children");

        return None;
    }
}
