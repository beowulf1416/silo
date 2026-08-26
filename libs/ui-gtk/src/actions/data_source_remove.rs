use tracing::debug;

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;

pub fn data_source_remove_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("data-source-remove", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |a, b| {
            debug!("data-source-remove action activated");
            // window.send(ApplicationMessage::Data);
        }
    ));

    return action;
}
