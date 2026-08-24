mod imp;
pub mod node;
pub mod postgres_plugin;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;

glib::wrapper! {
    pub struct PostgresConnectionEditor(ObjectSubclass<imp::PostgresConnectionEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl PostgresConnectionEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_main_window(&self, window: &MainWindow) {
        let imp = self.imp();
        imp.set_main_window(&window);
    }

    pub fn save_configuration(&self) {
        debug!("save_configuration");

        let imp = self.imp();

        let name = imp.entry_name.text().to_string();
        let host = imp.entry_host.text().to_string();
        let port = imp.entry_port.text().to_string();
        let db = imp.entry_db.text().to_string();
        let user = imp.entry_user.text().to_string();
        let pw = imp.entry_pw.text().to_string();

        let config = serde_json::json!({
            "name": name,
            "host": host,
            "port": port,
            "user": user,
            "pw": pw
        });
    }
}

impl Default for PostgresConnectionEditor {
    fn default() -> Self {
        return Self::new();
    }
}
