mod new_data_source_postgres_action;
mod new_workspace_action;

use gtk::prelude::ApplicationExt;
use relm4::{actions::*, main_application, prelude::*};
use std::rc::Rc;
use tracing::debug;

use crate::app::windows::main::{MainWindow, MainWindowMsg};

relm4::new_action_group!(pub ApplicationActionGroup, "app");
// relm4::new_action_group!(pub(super) WindowActionGroup, "win");

pub(crate) use new_data_source_postgres_action::{
    NewDataSourcePostGresAction, new_data_source_postgres_action,
};
pub(crate) use new_workspace_action::{NewWorkspaceAction, new_workspace_action};

relm4::new_stateless_action!(pub QuitAction, ApplicationActionGroup, "quit");
pub fn quit_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<QuitAction> {
    return RelmAction::<QuitAction>::new_stateless(move |_| {
        debug!("quitting...");
        relm4::main_application().quit();
        // sender.input(MainWindowMsg::Quit);
    });
}

relm4::new_stateless_action!(pub PreferencesAction, ApplicationActionGroup, "preferences");
pub fn preferences_action(
    sender: Rc<ComponentSender<MainWindow>>,
) -> RelmAction<PreferencesAction> {
    return RelmAction::<PreferencesAction>::new_stateless(move |_| {
        // PreferencesDialog::builder().launch(()).detach();
        debug!("preferences action");
    });
}

relm4::new_stateless_action!(pub AboutAction, ApplicationActionGroup, "about");
pub fn about_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<AboutAction> {
    return RelmAction::<AboutAction>::new_stateless(move |_| {
        // AboutDialog::builder().launch(()).detach();
        debug!("about action");
    });
}

relm4::new_stateless_action!(pub ShortcutsAction, ApplicationActionGroup, "shortcuts");
pub fn shortcuts_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<ShortcutsAction> {
    return RelmAction::<ShortcutsAction>::new_stateless(move |_| {
        // AboutDialog::builder().launch(()).detach();
        debug!("shortcuts action");
    });
}

// relm4::new_stateless_action!(pub DataStoreAddAction, ApplicationActionGroup, "data-store-add");
// relm4::new_stateless_action!(pub DataSourceAddPostgresAction, ApplicationActionGroup, "data-store-add-postgres");
