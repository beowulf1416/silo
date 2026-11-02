use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::sync::Arc;

use crate::components::main_window::MainWindow;

#[derive(Debug, Default)]
pub struct MySQLQueryEditor {
    pub window: RefCell<Option<MainWindow>>,
}

impl MySQLQueryEditor {
    pub fn set_main_window(&self, window: &MainWindow) {
        self.window.replace(Some(window.clone()));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MySQLQueryEditor {
    const NAME: &'static str = "MySQLQueryEditor";
    type Type = super::MySQLQueryEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for MySQLQueryEditor {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
    }
}

impl WidgetImpl for MySQLQueryEditor {}

impl BoxImpl for MySQLQueryEditor {}
