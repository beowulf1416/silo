mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct AuthWindow(ObjectSubclass<imp::AuthWindowImp>)
    @extends gtk::Widget, gtk::Window, adw::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl AuthWindow {
    pub fn new() -> Self {
        let window: Self = glib::Object::builder().build();

        return window;
    }
}
