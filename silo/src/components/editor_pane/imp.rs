use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use tracing::debug;

#[derive(Debug, Default)]
pub struct EditorPaneImpl {}

#[glib::object_subclass]
impl ObjectSubclass for EditorPaneImpl {
    const NAME: &'static str = "EditorPaneImpl";
    type Type = super::EditorPane;
    type ParentType = gtk::Box;
}

impl ObjectImpl for EditorPaneImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_vexpand(true);
        obj.set_hexpand(true);
    }
}

impl WidgetImpl for EditorPaneImpl {}

impl BoxImpl for EditorPaneImpl {}

impl EditorPaneImpl {}
