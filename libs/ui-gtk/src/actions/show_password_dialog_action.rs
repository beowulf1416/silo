use tracing::{debug, error};

use gtk::{gio, glib, prelude::*};

use crate::components::auth_window::AuthWindow;
use crate::components::main_window::MainWindow;

pub fn show_password_dialog_action(mw: &MainWindow) -> gio::SimpleAction {
    debug!("show_password_dialog_action");

    let action = gio::SimpleAction::new("show-password-dialog", None);
    action.connect_activate(glib::clone!(
        #[weak]
        mw,
        move |_action, _target| {
            debug!("show_password_dialog_action activated");

            let aw = AuthWindow::new();
            // aw.set_parent(&mw);
            aw.set_transient_for(Some(&mw));
            aw.present();
        }
    ));

    return action;
}
