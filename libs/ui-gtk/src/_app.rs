use tracing::debug;

use gtk::{Application, gdk, gio, glib, prelude::*};
use std::rc::Rc;

use crate::APP_ID;
use crate::components::main_window::MainWindow;
use silo_base::Silo;

pub struct App {
    silo: Rc<Silo>,
}

impl App {
    pub fn run(silo: &Silo) {
        debug!("starting...");

        gtk::init().unwrap();

        let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/org.devphilplus.silo.gresource"
        )));
        let resource = gio::Resource::from_data(&resource_bytes).unwrap();
        gio::resources_register(&resource);

        // gtk::Window::set_default_icon_name("org.devphilplus.silo");
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_startup(|_app| {
            let display = gdk::Display::default().unwrap();
            let theme = gtk::IconTheme::for_display(&display);
            theme.add_resource_path("/org/devphilplus/silo/images");
        });

        // let window_main = MainWindow {};
        app.connect_activate(MainWindow::build);
        app.run();
    }
}
