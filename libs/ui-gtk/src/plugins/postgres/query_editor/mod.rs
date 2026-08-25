mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;

glib::wrapper! {
    pub struct PostgresQueryEditor(ObjectSubclass<imp::PostgresQueryEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl PostgresQueryEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_main_window(&self, window: &MainWindow) {
        let imp = self.imp();
        imp.set_main_window(&window);
    }
}

impl Default for PostgresQueryEditor {
    fn default() -> Self {
        return Self::new();
    }
}
