mod imp;

use tracing::debug;

use gtk::{
    gio::{self, prelude::*},
    glib,
    prelude::*,
    subclass::prelude::*,
};

// use crate::components::main_window::MainWindow;
use silo_plugin::{ApplicationMessage, StatusMessage};

glib::wrapper! {
    pub struct QueryEditor(ObjectSubclass<imp::QueryEditor>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl QueryEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn with_model(sources: &gio::ListStore) -> Self {
        let obj: Self = glib::Object::builder().build();
        let imp = obj.imp();
        imp.set_model(sources.clone());

        return obj;
    }

    pub fn set_sender(
        &self,
        sender: async_channel::Sender<ApplicationMessage>,
        sender_status: async_channel::Sender<StatusMessage>,
    ) {
        let imp = self.imp();
        imp.sender.replace(Some(sender));

        imp.sender_status.replace(Some(sender_status));
    }

    pub fn set_data_sources(&self, sources: gio::ListStore) {
        let imp = self.imp();
        imp.set_model(sources);
    }

    pub fn execute(&self) {
        let imp = self.imp();
        imp.execute();
    }
}

impl Default for QueryEditor {
    fn default() -> Self {
        return Self::new();
    }
}
