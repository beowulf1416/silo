use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::sync::Arc;

// use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;

#[derive(Debug, Default)]
pub struct PostgresQueryEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,
}

impl PostgresQueryEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }
}

#[glib::object_subclass]
impl ObjectSubclass for PostgresQueryEditor {
    const NAME: &'static str = "PostgresQueryEditor";
    type Type = super::PostgresQueryEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for PostgresQueryEditor {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
    }
}

impl WidgetImpl for PostgresQueryEditor {}

impl BoxImpl for PostgresQueryEditor {}
