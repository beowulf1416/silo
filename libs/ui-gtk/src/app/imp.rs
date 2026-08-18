use std::cell::OnceCell;
use std::rc::Rc;

use gtk::{gdk, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;
use silo_base::Silo;

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::App)]
pub struct App {
    // #[property(get, set, construct_only)]
    // pub silo: OnceCell<Rc<Silo>>,
}

impl App {}

#[glib::object_subclass]
impl ObjectSubclass for App {
    const NAME: &'static str = "Silo";
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for App {}

impl ApplicationImpl for App {
    fn activate(&self) {
        self.parent_activate();

        let app = self.obj();
        let window = if let Some(window) = app.active_window() {
            window
        } else {
            let window = MainWindow::new(&app);
            window.upcast()
        };

        // let mw = MainWindow::new(&*self.obj());

        // mw.present();
        //
        window.present();
    }

    fn startup(&self) {
        self.parent_startup();

        let display = gdk::Display::default().unwrap();
        let theme = gtk::IconTheme::for_display(&display);
        theme.add_resource_path("/org/devphilplus/silo/images");
    }
}

impl GtkApplicationImpl for App {}
