mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::app::App;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindowImpl>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl MainWindow {
    pub fn app(&self) -> Option<App> {
        return self.application().and_downcast_ref::<App>().cloned();
    }
}
