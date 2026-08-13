pub mod actions;
pub mod components;
pub mod models;
pub mod windows;

use tracing::debug;
// use main_window::MainWindow;

use std::rc::Rc;


// use gtk::{Application, glib};
use gtk::{ApplicationWindow, prelude::{ *, ApplicationExt}};
// use gtk::prelude::{
//     ActionableExt, ApplicationExt, ButtonExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
// };
use relm4::{actions::{*, AccelsPlus}, main_application, prelude::*};

// use crate::app::main_window::MainWindowMsg;
use crate::app::windows::main::{MainWindow, MainWindowMsg};

use crate::app::actions::*;
// use crate::app::actions::ApplicationActionGroup;
// use crate::app::actions::DataSourceAddPostgresAction;
// use crate::app::actions::DataStoreAddAction;
// use crate::app::actions::QuitAction;
//

// use crate::app::actions::WindowActionGroup;

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

        let gtk_app = main_application();
        let app = RelmApp::from_app(gtk_app);

        // // build the window
        // let controller = MainWindow::builder()
        //     .launch(())
        //     .detach();

        // let main_window = controller.widget();
        // let sender: Rc<ComponentSender<MainWindow>> = Rc::new(controller.sender());


        // actions
        // let mut action_group = RelmActionGroup::<ApplicationActionGroup>::new();



        // let quit_action = {
        //     RelmAction::<QuitAction>::new_stateless(move |_| {
        //         debug!("quitting...");
        //         main_application().quit();
        //     })
        // };
        // action_group.add_action(quit_action);

        // let quit_action = quit_action(sender.clone());
        // action_group.add_action(quit_action);
        // gtk_app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        // let new_workspace_action = new_workspace_action(sender.clone());
        // action_group.add_action(new_workspace_action);

        // action_group.register_for_main_application();

        app.visible_on_activate(false).run::<MainWindow>(());
    }
}
