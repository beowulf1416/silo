use tracing::debug;

use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    components::data_sources_view::node::Node, plugins::postgres::nodes::schema_node::SchemaNode,
};

#[derive(Debug, Clone)]
pub struct PostgresDataSourceNode {
    name: String,
}

impl PostgresDataSourceNode {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}

impl Node for PostgresDataSourceNode {
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    fn clone_box(&self) -> Box<dyn Node> {
        debug!("PostgresDataSourceNode::clone_box");

        return Box::new(self.clone());
    }

    fn children(&self) -> Option<gio::ListStore> {
        debug!("PostgresDataSourceNode::children");

        let store = gio::ListStore::new::<BoxedAnyObject>();

        // test data
        let boxed: Box<dyn Node> = Box::new(SchemaNode::new("public"));
        store.append(&glib::BoxedAnyObject::new(boxed));

        let boxed: Box<dyn Node> = Box::new(SchemaNode::new("eas"));
        store.append(&glib::BoxedAnyObject::new(boxed));

        return Some(store);
    }
}
