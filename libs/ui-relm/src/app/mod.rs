pub mod actions;
pub mod components;
pub mod models;
pub mod windows;

use tracing::debug;
// use main_window::MainWindow;

// use gtk::{Application, glib};
use gtk::{ApplicationWindow, prelude::*};
use relm4::{actions::*, main_application, prelude::*};

// use crate::app::main_window::MainWindowMsg;
use crate::app::windows::main::{MainWindow, MainWindowMsg};

use crate::app::actions::ApplicationActionGroup;
use crate::app::actions::DataSourceAddPostgresAction;
use crate::app::actions::DataStoreAddAction;
use crate::app::actions::QuitAction;

const APP_ID: &str = "org.devphilplus.silo";

// relm4::new_action_group!(ApplicationActionGroup, "app");
// relm4::new_stateless_action!(DataStoreAddAction, ApplicationActionGroup, "data-store-add");
// relm4::new_stateless_action!(
//     DataSourceAddPostgresAction,
//     ApplicationActionGroup,
//     "data-source-add-postgres"
// );

pub struct App {
    // app: RelmApp<MainWindowMsg>,
    // window: ApplicationWindow,
}

impl App {
    pub fn run() {
        debug!("starting...");

        gtk::init().unwrap();
        gtk::Window::set_default_icon_name(APP_ID);

        let app = main_application();
        let app = RelmApp::from_app(app);

        // actions
        let mut action_group = RelmActionGroup::<ApplicationActionGroup>::new();
        let data_store_add_action = {
            RelmAction::<DataStoreAddAction>::new_stateless(move |_| {
                debug!("data store add action");
            })
        };
        action_group.add_action(data_store_add_action);

        let data_source_add_postgres_action = {
            RelmAction::<DataSourceAddPostgresAction>::new_stateless(move |_| {
                debug!("data source add postgres action");
            })
        };
        action_group.add_action(data_source_add_postgres_action);

        let quit_action = {
            RelmAction::<QuitAction>::new_stateless(move |_| {
                debug!("quitting...");
                main_application().quit();
            })
        };
        action_group.add_action(quit_action);

        action_group.register_for_main_application();

        app.visible_on_activate(false).run::<MainWindow>(());
    }
}
