pub mod actions;
pub mod components;
pub mod models;
pub mod windows;

use tracing::debug;
// use main_window::MainWindow;

// use std::env;
use std::rc::Rc;

// use gtk::{Application, glib};
use gtk::{ApplicationWindow, gdk, gio, glib, prelude::ApplicationExt};
// use gtk::prelude::{
//     ActionableExt, ApplicationExt, ButtonExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt,
// };
use relm4::{
    actions::{AccelsPlus, *},
    main_application,
    prelude::*,
};

use crate::app::windows::main::{MainWindow, MainWindowMsg};
use silo_base::Silo;

// use crate::app::actions::*;

use crate::APP_ID;
// const APP_ID: &str = "org.devphilplus.silo";

pub struct App {
    silo: Rc<Silo>,
}

impl App {
    pub fn run(silo: Silo) {
        debug!("starting...");

        gtk::init().unwrap();

        let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/org.devphilplus.silo.gresource"
        )));
        let resource = gio::Resource::from_data(&resource_bytes).unwrap();
        gio::resources_register(&resource);

        gtk::Window::set_default_icon_name("org.devphilplus.silo");

        let gtk_app = main_application();

        gtk_app.connect_startup(|app| {
            let display = gdk::Display::default().unwrap();
            let theme = gtk::IconTheme::for_display(&display);
            theme.add_resource_path("/org/devphilplus/silo/images");
        });

        let app = RelmApp::from_app(gtk_app);

        // relm4_icons::initialize_icons();

        // gio::resources_register_include!("org.devphilplus.silo.gresource")
        //     .expect("failed to register resources");

        // let display = gdk::Display::default().unwrap();
        // let theme = gtk::IconTheme::for_display(&display);
        // theme.add_resource_path("/org/devphilplus/silo/images");

        app.visible_on_activate(false).run::<MainWindow>(silo);
    }
}
