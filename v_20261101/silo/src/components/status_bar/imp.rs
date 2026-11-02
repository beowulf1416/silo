use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use tracing::debug;

#[derive(Debug, Default)]
pub struct StatusBarImpl {}

#[glib::object_subclass]
impl ObjectSubclass for StatusBarImpl {
    const NAME: &'static str = "StatusBarImpl";
    type Type = super::StatusBar;
    type ParentType = gtk::Box;
}

impl ObjectImpl for StatusBarImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_vexpand(true);
        obj.set_hexpand(false);
    }
}

impl WidgetImpl for StatusBarImpl {}

impl BoxImpl for StatusBarImpl {}

impl StatusBarImpl {}
