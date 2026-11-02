use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use tracing::debug;

#[derive(Debug, Default)]
pub struct EditorPaneImpl {}

impl EditorPaneImpl {}

#[glib::object_subclass]
impl ObjectSubclass for EditorPaneImpl {
    const NAME: &'static str = "EditorPaneImpl";
    type Type = super::EditorPane;
    type ParentType = gtk::Box;
}

impl ObjectImpl for EditorPaneImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();

        let obj = self.obj();
        obj.set_vexpand(true);
        obj.set_hexpand(true);
        obj.append(&container);
    }
}

impl WidgetImpl for EditorPaneImpl {}

impl BoxImpl for EditorPaneImpl {}
