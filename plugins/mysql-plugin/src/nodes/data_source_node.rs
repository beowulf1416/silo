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

    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>> {
        return Err(anyhow::anyhow!("//todo not implemented"));
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
}

#[async_trait]
impl DataSourceNode for MySQLDataSourceNode {
    async fn query(&self, sql: &str) -> anyhow::Result<()> {
        return Err(anyhow::anyhow!("//todo not implemented"));
    }

    fn get_configuration(&self) -> serde_json::Value {
        return serde_json::json!({
            "name": self.settings.name.clone(),
            "host": self.settings.host.clone(),
            "port": self.settings.port.clone(),
            "user": self.settings.user.clone(),
            "pw": self.settings.pw.clone(),
        });
    }
}
