mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::app::App;

pub enum MainWindowMessages {
    Close,
    CloseRequested,
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindowImpl>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl MainWindow {
    pub fn new(app: &App) -> Self {
        let window: Self = glib::Object::builder().property("application", app).build();
        window.imp().restore_settings();

        return window;
    }

    pub fn app(&self) -> Option<App> {
        return self.application().and_downcast_ref::<App>().cloned();
    }

    pub fn send(&self, msg: MainWindowMessages) {
        if let Some(sender) = self.imp().sender.borrow().as_ref() {
            let _ = sender.send_blocking(msg);
        }
    }
}
