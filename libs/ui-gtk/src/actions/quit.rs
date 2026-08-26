use tracing::debug;

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;

use silo_plugin::ApplicationMessage;

pub fn quit_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("quit", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |a, b| {
            debug!("quit action activated");
            window.send(ApplicationMessage::CloseRequested);
        }
    ));

    return action;
}
