mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::app::App;
use crate::components::main_window::{MainWindow, MainWindowMessages};

glib::wrapper! {
    pub struct CloseRequestDialog(ObjectSubclass<imp::CloseRequestDialogImpl>)
    @extends gtk::Widget, gtk::Window, adw::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl CloseRequestDialog {
    pub fn new(app: &App, parent: &MainWindow) -> Self {
        let window: Self = glib::Object::builder().property("application", app).build();
        window.imp().set_parent(parent);

        return window;
    }
}
