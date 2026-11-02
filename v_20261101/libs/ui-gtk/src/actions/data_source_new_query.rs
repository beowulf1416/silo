use tracing::debug;

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;

pub fn data_source_new_query_action(window: &MainWindow) -> gio::SimpleAction {
    let action = gio::SimpleAction::new_stateful(
        "data-source-new-query",
        Some(glib::VariantTy::STRING),
        &glib::Variant::from("text"),
    );
    action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_action, parameter| {
            debug!("data-source-new-query");

            if let Some(value) = parameter {
                if let Some(plugin_name) = value.get::<String>() {
                    debug!("parameter {}", plugin_name);

                    window.send(ApplicationMessage::NewQueryEditorRequested(plugin_name));
                }
            }
        }
    ));

    return action;
}
