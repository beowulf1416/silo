use tracing::debug;

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;

pub fn data_source_add_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new_stateful(
        "data-source-add",
        Some(glib::VariantTy::STRING),
        &glib::Variant::from("text"),
    );
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_action, parameter| {
            debug!("data-source-add action activated");

            if let Some(value) = parameter {
                if let Some(plugin_name) = value.get::<String>() {
                    debug!("parameter {}", plugin_name);

                    window.send(ApplicationMessage::NewDataSourceRequested(plugin_name));
                }
            }

            // let state: String = action
            //     .state()
            //     .and_then(|v| v.get())
            //     .unwrap_or("text".to_string());
            // debug!("state {:?}", state);
        }
    ));

    return action;
}
