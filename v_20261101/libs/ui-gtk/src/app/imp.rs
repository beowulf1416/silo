use tracing::debug;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::{CssProvider, gdk, glib, prelude::*, subclass::prelude::*};

use crate::components::main_window::MainWindow;
use silo_base::Silo;

// use crate::plugins::PluginRegistry;
// type PluginFactory = fn() -> Box<dyn Plugin>;
use silo_plugin::plugin::{Plugin, PluginRegistry};

// #[derive(Debug, Default, glib::Properties)]
// #[properties(wrapper_type = super::App)]
#[derive(Debug, Default)]
pub struct App {
    // #[property(get, set, construct_only)]
    // pub workspace_path: OnceCell<String>, // #[property(get, set, construct_only)]
    // pub silo: RefCell<Silo>,
    pub registry: RefCell<Option<PluginRegistry>>,
    pub workspace_path: RefCell<Option<String>>,
    pub window: RefCell<Option<MainWindow>>,
}

impl App {
    // pub fn set_silo(&self, silo: Silo) {
    //     self.silo.replace(silo);
    // }

    pub fn set_workspace_path(&self, new_path: String) {
        self.workspace_path.replace(Some(new_path));

        // open connections file
    }

    fn set_main_window(&self, window: MainWindow) {
        self.window.replace(Some(window));
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
        self.set_main_window(
            window
                .clone()
                .downcast::<MainWindow>()
                .expect("//todo MainWindow"),
        );

        window.present();
    }

    fn startup(&self) {
        self.parent_startup();

        let display = gdk::Display::default().unwrap();
        let theme = gtk::IconTheme::for_display(&display);
        theme.add_resource_path("/org/devphilplus/silo/images");

        let provider = CssProvider::new();
        provider.load_from_resource("/org/devphilplus/silo/css/styles.css");
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

impl GtkApplicationImpl for App {}
