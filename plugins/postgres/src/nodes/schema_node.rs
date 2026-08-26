use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::components::data_sources_view::node::Node;
// use crate::plugins::postgres::nodes::schema_functions_node::SchemaFunctionsNode;
// use crate::plugins::postgres::nodes::schema_procedures_node::SchemaProceduresNode;
// use crate::plugins::postgres::nodes::schema_tables_node::SchemaTablesNode;

use crate::nodes::schema_functions_node::SchemaFunctionsNode;
use crate::nodes::schema_procedures_node::SchemaProceduresNode;
use crate::nodes::schema_tables_node::SchemaTablesNode;
use silo_plugin::node::Node;

#[derive(Debug, Clone)]
pub struct SchemaNode {
    pub name: String,
}

impl SchemaNode {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }
}

impl Node for SchemaNode {
    fn name(&self) -> &str {
        return &self.name.as_str();
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    fn children(&self) -> Option<gio::ListStore> {
        debug!("SchemaNode::children");

        let store = gio::ListStore::new::<glib::BoxedAnyObject>();

        let boxed: Box<dyn Node> = Box::new(SchemaTablesNode {});
        store.append(&glib::BoxedAnyObject::new(boxed));

        let boxed: Box<dyn Node> = Box::new(SchemaProceduresNode {});
        store.append(&glib::BoxedAnyObject::new(boxed));

        let boxed: Box<dyn Node> = Box::new(SchemaFunctionsNode {});
        store.append(&glib::BoxedAnyObject::new(boxed));

        return Some(store);
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        menu.append_item(&item);

        return Some(menu);
    }
}
