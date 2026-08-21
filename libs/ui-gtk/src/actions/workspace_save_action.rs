use tracing::{debug, error};

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::{MainWindow, MainWindowInputMessage};

pub fn workspace_save_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("workspace-save", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_action, _parameter| {
            debug!("workspace-save action activated");
        }
    ));

    return action;
}
