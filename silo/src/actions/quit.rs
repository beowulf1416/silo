use gtk::{gio, glib, prelude::*};
use tracing::debug;

use crate::components::app_window::AppWindow;
use crate::components::app_window::AppWindowMessages;

pub fn quit_action(window: &AppWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("quit", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_action, _target| {
            debug!("quit action activated");
            window.send(AppWindowMessages::CloseRequested);
        }
    ));

    return action;
}
