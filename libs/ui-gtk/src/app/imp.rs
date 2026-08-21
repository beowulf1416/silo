use tracing::debug;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::{gdk, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;
use silo_base::Silo;

use crate::plugins::PluginRegistry;
// type PluginFactory = fn() -> Box<dyn Plugin>;

// #[derive(Debug, Default, glib::Properties)]
// #[properties(wrapper_type = super::App)]
#[derive(Debug, Default)]
pub struct App {
    // #[property(get, set, construct_only)]
    // pub workspace_path: OnceCell<String>, // #[property(get, set, construct_only)]
    // pub silo: RefCell<Silo>,
    pub registry: RefCell<Option<PluginRegistry>>,
    pub workspace_path: RefCell<Option<String>>,
}

impl App {
    // pub fn set_silo(&self, silo: Silo) {
    //     self.silo.replace(silo);
    // }

    pub fn set_workspace_path(&self, new_path: String) {
        self.workspace_path.replace(Some(new_path));

        // open connections file
    }
}

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
