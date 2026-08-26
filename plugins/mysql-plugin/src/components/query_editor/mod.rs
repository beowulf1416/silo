mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::ApplicationMessage;

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

    pub fn set_sender(&self, sender: async_channel::Sender<ApplicationMessage>) {
        let imp = self.imp();
        imp.sender.replace(Some(sender));
    }
}

impl Default for MySQLQueryEditor {
    fn default() -> Self {
        return Self::new();
    }
}
