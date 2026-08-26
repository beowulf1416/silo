mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::ApplicationMessage;

glib::wrapper! {
    pub struct MySQLConnectionEditor(ObjectSubclass<imp::MySQLConnectionEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl MySQLConnectionEditor {
    pub fn new() -> Self {
        let obj = glib::Object::builder().build();

        return obj;
    }

    // pub fn set_main_window(&self, window: &MainWindow) {
    //     let imp = self.imp();
    //     imp.set_main_window(&window);
    // }
    pub fn set_sender(&self, sender: async_channel::Sender<ApplicationMessage>) {
        debug!("set_sender {:?}", sender);
        let imp = self.imp();
        imp.sender.replace(Some(sender));
    }
}

impl Default for MySQLConnectionEditor {
    fn default() -> Self {
        return Self::new();
    }
}
