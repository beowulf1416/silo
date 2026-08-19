use tracing::debug;

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::{MainWindow, MainWindowInputMessage};

pub fn data_source_remove_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("data-source-remove", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |a, b| {
            debug!("data-source-remove action activated");
            // window.send(MainWindowInputMessage::CloseRequested);
        }
    ));

    return action;
}
