mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;

glib::wrapper! {
    pub struct MySQLConnectionEditor(ObjectSubclass<imp::MySQLConnectionEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl MySQLConnectionEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_main_window(&self, window: &MainWindow) {
        let imp = self.imp();
        imp.set_main_window(&window);
    }
}

impl Default for MySQLConnectionEditor {
    fn default() -> Self {
        return Self::new();
    }
}
