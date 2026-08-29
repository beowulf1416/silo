use async_trait::async_trait;
use gtk::{
    gio,
    glib::{self, BoxedAnyObject},
    prelude::*,
    subclass::prelude::*,
};
use std::sync::Arc;

use tracing::debug;

use crate::nodes::ConnectionSettings;
use crate::nodes::schema_functions_node::SchemaFunctionsNode;
use crate::nodes::schema_procedures_node::SchemaProceduresNode;
use crate::nodes::schema_tables_node::SchemaTablesNode;
use silo_plugin::node::{DataSourceNode, Node};

#[derive(Debug, Clone)]
pub struct MySQLDataSourceNode {
    name: String,
    settings: Arc<ConnectionSettings>,
}

impl MySQLDataSourceNode {
    pub fn new(name: &str, settings: ConnectionSettings) -> Self {
        return Self {
            name: name.to_string(),
            settings: Arc::new(settings),
        };
    }
}

#[async_trait]
impl Node for MySQLDataSourceNode {
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    // fn clone_box(&self) -> Box<dyn Node> {
    //     debug!("MySQLDataSourceNode::clone_box");

    //     return Box::new(self.clone());
    // }

    fn children(&self) -> Option<Vec<Arc<dyn Node>>> {
        debug!("MySQLDataSourceNode::children");

        let mut nodes: Vec<Arc<dyn Node>> = vec![];

        // test data
        let boxed: Arc<dyn Node> = Arc::new(SchemaTablesNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        let boxed: Arc<dyn Node> = Arc::new(SchemaProceduresNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        let boxed: Arc<dyn Node> = Arc::new(SchemaFunctionsNode::new(Arc::clone(&self.settings)));
        nodes.push(boxed);

        return Some(nodes);
    }

    async fn children_async(&self) -> Result<Option<Vec<Arc<dyn Node>>>, &'static str> {
        return Err("//todo not implemented");
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Edit"), Some("win.data-source-edit::mysql"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("Refresh"), Some("win.data-source-refresh::mysql"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        let item = gio::MenuItem::new(Some("Remove"), Some("win.data-source-remove::mysql"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return Some(menu);
    }

    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>> {
        return Some(Arc::new(self.clone()));
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}

#[async_trait]
impl DataSourceNode for MySQLDataSourceNode {
    async fn query(&self, sql: &str) -> Result<(), &'static str> {
        return Err("//todo not implemented");
    }
}
