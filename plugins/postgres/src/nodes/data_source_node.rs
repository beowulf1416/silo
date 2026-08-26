use tracing::debug;

use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};

use crate::nodes::schema_node::SchemaNode;
use silo_plugin::node::Node;

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

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Edit"), Some("win.data-source-edit::postgres"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        let item = gio::MenuItem::new(Some("Remove"), Some("win.data-source-remove::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return Some(menu);
    }
}
