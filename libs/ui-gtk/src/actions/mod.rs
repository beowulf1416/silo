use gtk::gio::prelude::ActionMapExt;

// use crate::{
//     actions::workspace_open_action::workspace_open_action, components::main_window::MainWindow,
// };

use crate::components::main_window::MainWindow;

pub mod data_source_add;
pub mod data_source_remove;
pub mod quit;
pub mod workspace_open_action;
pub mod workspace_save_action;

pub fn setup_actions(window: &MainWindow) {
    let action = quit::quit_action(&window);
    window.add_action(&action);

    let action = data_source_add::data_source_add_action(&window);
    window.add_action(&action);

    let action = data_source_remove::data_source_remove_action(&window);
    window.add_action(&action);

    let action = workspace_open_action::workspace_open_action(&window);
    window.add_action(&action);

    let action = workspace_save_action::workspace_save_action(&window);
    window.add_action(&action);
}
