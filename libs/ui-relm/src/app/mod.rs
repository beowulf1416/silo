// pub mod data_store_window;
// pub mod main_window;

pub mod components;
pub mod windows;

// use main_window::MainWindow;

// use gtk::{Application, glib};
use gtk::{ApplicationWindow, prelude::*};
use relm4::{main_application, prelude::*};

// use crate::app::main_window::MainWindowMsg;
use crate::app::windows::main::MainWindow;

const APP_ID: &str = "org.devphilplus.silo";

pub struct App {
    // app: RelmApp<MainWindowMsg>,
    // window: ApplicationWindow,
}

impl App {
    pub fn run() {
        gtk::init().unwrap();
        gtk::Window::set_default_icon_name(APP_ID);

        let app = main_application();
        let app = RelmApp::from_app(app);
        app.visible_on_activate(false).run::<MainWindow>(());
    }
}
