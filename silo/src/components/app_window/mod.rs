mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::{error, info};

use silo_api::{Silo, window::MainWindow};

use crate::app::App;

#[derive(Debug, Clone)]
pub enum AppWindowMessages {
    Close,
    CloseRequested,
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindowImpl>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

impl AppWindow {
    pub fn new(app: &App) -> Self {
        info!("AppWindow::new");

        let window: Self = glib::Object::builder().property("application", app).build();

        return window;
    }

    pub fn send(&self, msg: AppWindowMessages) {
        info!("AppWindow::send");

        let imp = self.imp();
        if let Some(sender) = imp.sender.borrow().as_ref() {
            if let Err(e) = sender.try_send(msg.clone()) {
                error!("unable to send AppWindowMessage {:?} {:?}", msg, e);
            }
        }
    }

    pub fn set_silo(&self, silo: &Arc<dyn Silo>) {
        let imp = self.imp();
        // imp.silo.replace(Some(silo));
        imp.set_silo(&silo);
    }

    pub fn show_password_dialog(&self) {
        info!("AppWindow::show_password_dialog()");

        let imp = self.imp();
        imp.show_password_dialog();
    }
}

// impl MainWindow for AppWindow {
//     fn show_password_dialog(&self) {
//         info!("AppWindow::show_password_dialog()");
//     }
// }
