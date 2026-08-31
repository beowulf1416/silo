mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::plugins::Plugin;
use crate::components::query_editor::QueryEditor;
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

    pub fn get_current_query_editor(&self) -> Option<QueryEditor> {
        let imp = self.imp();
        debug!(
            "get_current_editor {:?}",
            imp.nb.nth_page(imp.nb.current_page())
        );

        if let Some(widget) = imp.nb.nth_page(imp.nb.current_page()) {
            if let Ok(query_editor) = widget.downcast::<QueryEditor>() {
                return Some(query_editor);
            }
        }

        return None;
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
