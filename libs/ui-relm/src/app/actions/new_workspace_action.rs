use tracing::debug;

use super::*;

use std::rc::Rc;
use gtk::prelude::*;
use relm4::{actions::*, main_application, prelude::*};

use crate::app::windows::main::MainWindow;

relm4::new_stateless_action!(pub NewWorkspaceAction, ApplicationActionGroup, "new-workspace");

pub fn new_workspace_action(sender: Rc<ComponentSender<MainWindow>>, parent: adw::ApplicationWindow) -> RelmAction<NewWorkspaceAction> {
    let window = parent.upcast::<gtk::Window>();

    return RelmAction::<NewWorkspaceAction>::new_stateless(move |_| {
        debug!("new-workspace action triggered");

        let dialog = gtk::FileDialog::builder()
            .title("Select Workspace")
            // .parent(&window)
            // .transient_for(&window)
            .modal(true)
            .build();

        // let window = parent.clone().upcast::<gtk::Window>();
        let window = window.clone();

        let sender = sender.clone();
        dialog.select_folder(Some(&window), gtk::gio::Cancellable::NONE, move |result| {
            debug!("select_folder result: {:?}", result);
        });
    });
}
