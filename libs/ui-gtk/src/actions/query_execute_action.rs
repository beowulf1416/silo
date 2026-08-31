use tracing::{debug, error};

use gtk::{gio, glib, prelude::*};

use crate::components::main_window::MainWindow;

pub fn query_execute_action(mw: &MainWindow) -> gio::SimpleAction {
    debug!("query_execute_action");

    let action = gio::SimpleAction::new("query-execute", None);
    action.connect_activate(glib::clone!(
        #[weak]
        mw,
        move |_action, _target| {
            // debug!("query_execute_action activated {:?} {:?}", a, b);

            let ev = mw.editor_view();
            if let Some(query_editor) = ev.get_current_query_editor() {
                debug!("query_editor: {:?}", query_editor);

                // action.set_enabled(false);
                query_editor.execute();
                // action.set_enabled(true);
            }
        }
    ));

    return action;
}
