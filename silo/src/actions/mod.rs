pub mod quit;

use gtk::gio::prelude::ActionMapExt;

use crate::components::app_window::AppWindow;

pub fn setup_actions(window: &AppWindow) {
    let action = quit::quit_action(&window);
    window.add_action(&action);
}
