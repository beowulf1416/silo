mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::plugins::Plugin;

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

    pub fn add_editor(&self, plugin: Box<dyn Plugin>) {
        let imp = self.imp();
        imp.add_editor(plugin);
    }
}

impl Default for EditorView {
    fn default() -> Self {
        return Self::new();
    }
}
