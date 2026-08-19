use tracing::{debug, error};

use async_channel::Sender;
use std::cell::Ref;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct EditorView {
    pub nb: gtk::Notebook,
}

impl EditorView {
    pub fn add_editor(&self, widget: gtk::Widget) {}
}

#[glib::object_subclass]
impl ObjectSubclass for EditorView {
    const NAME: &'static str = "EditorView";
    type Type = super::EditorView;
    type ParentType = gtk::Box;
}

impl ObjectImpl for EditorView {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        // let nb = gtk::Notebook::builder().hexpand(true).vexpand(true).build();
        // content_box.append(&nb);

        self.nb.set_hexpand(true);
        self.nb.set_vexpand(true);
        content_box.append(&self.nb);

        obj.append(&content_box);
    }
}

impl WidgetImpl for EditorView {}

impl BoxImpl for EditorView {}
