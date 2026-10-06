use gtk::{glib, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use tracing::debug;

use crate::app::registry;
use crate::components::main_window;

#[derive(Debug, Default)]
pub struct AppImpl {
    pub(super) workspace_path: RefCell<Option<String>>,
    pub(super) registry: RefCell<Option<registry::PluginRegistry>>,
    pub(super) connections: RefCell<Option<HashMap<String, Connection>
}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = crate::APP_NAME;
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppImpl {}

impl ApplicationImpl for AppImpl {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();

        let window = main_window::MainWindow::new(&obj);

        self.init_plugins();
        self.init_data_sources();

        window.present();
    }
}

impl GtkApplicationImpl for AppImpl {}

impl AppImpl {
    pub(super) fn set_workspace_path(&self, path: &str) {
        self.workspace_path.borrow_mut().replace(path.to_string());
    }

    pub(super) fn init_plugins(&self) {
        debug!("AppImpl::init_plugins");

        let mut registry = registry::PluginRegistry::new();

        let plugin = postgres_plugin::create_plugin();
        if let Err(e) = plugin.init() {
            debug!("Failed to initialize plugin: {:?}", e);
            // return;
        } else {
            registry.register(plugin);
        }

        self.registry.borrow_mut().replace(registry);

        // let plugins = vec!["postgres"];
        // for plugin in plugins {}
    }

    pub fn init_connections(&self) {
        debug!("AppImpl::init_connections");
    }

    pub fn init_data_sources(&self) {
        debug!("AppImpl::init_data_sources");
    }
}
