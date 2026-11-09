mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use tracing::{error, info};

// use silo_api::AppWindowMessages;

// use crate::app::App;
// use crate::components::app_window::AppWindow;

glib::wrapper! {
    pub struct PasswordDialog(ObjectSubclass<imp::PasswordDialogImpl>)
    @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl PasswordDialog {
    pub fn new() -> Self {
        info!("PasswordDialog::new()");

        let mut builder = glib::Object::builder();

        // if let Some(app) = parent.application() {
        //     builder = builder.property("application", app);
        // } else {
        //     error!("PasswordDialog::new() unable to set application property on window object");
        // }

        let window = builder.build();
        return window;
    }
}
