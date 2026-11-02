mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;

glib::wrapper! {
    pub struct MySQLQueryEditor(ObjectSubclass<imp::MySQLQueryEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl MySQLQueryEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_main_window(&self, window: &MainWindow) {
        let imp = self.imp();
        imp.set_main_window(&window);
    }
}

impl Default for MySQLQueryEditor {
    fn default() -> Self {
        return Self::new();
    }
}
