mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::App;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

// #[gtk::template_callbacks]
impl MainWindow {
    pub fn new(application: &App) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
