mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::plugins::Plugin;
use silo_plugin::ApplicationMessage;

glib::wrapper! {
    pub struct EditorView(ObjectSubclass<imp::EditorView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl EditorView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn add_editor(
        &self,
        display_name: &str,
        widget: gtk::Widget,
        sender: &async_channel::Sender<ApplicationMessage>,
    ) {
        let imp = self.imp();
        imp.add_editor(&display_name, widget, &sender);
    }

    pub fn remove_editor(&self, page: Option<u32>) {
        let imp = self.imp();
        imp.remove_editor(page);
    }
}

impl Default for EditorView {
    fn default() -> Self {
        return Self::new();
    }
}
