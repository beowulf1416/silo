use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::components::main_window;

#[derive(Debug, Default)]
pub struct AppImpl {}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = crate::APP_NAME;
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppImpl {}

impl ApplicationImpl for AppImpl {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();

        let window = main_window::MainWindow::new(&obj);
        window.present();
    }
}

impl GtkApplicationImpl for AppImpl {}

impl AppImpl {}
