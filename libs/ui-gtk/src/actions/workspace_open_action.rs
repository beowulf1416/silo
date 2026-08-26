use tracing::{debug, error};

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;

use silo_plugin::ApplicationMessage;

pub fn workspace_open_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("workspace-open", None);
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_action, _parameter| {
            debug!("workspace-open action activated");

            let dialog = gtk::FileDialog::builder()
                .title("Select Workspace")
                // .parent(&window)
                // .transient_for(&window)
                .modal(true)
                .build();

            let window_clone = window.clone();
            dialog.select_folder(Some(&window), gtk::gio::Cancellable::NONE, move |result| {
                debug!("select_folder result: {:?}", result);

                match result {
                    Err(e) => {
                        error!("select_folder error: {:?}", e);
                    }
                    Ok(path) => {
                        debug!("select_folder path: {:?}", path);
                        if let Some(path) = path.path() {
                            let path_string = path.to_string_lossy().into_owned();
                            window_clone.send(ApplicationMessage::WorkspaceChanged(path_string));
                        } else {
                            error!("select_folder path is None");
                        }
                    }
                }
            });
        }
    ));

    return action;
}
