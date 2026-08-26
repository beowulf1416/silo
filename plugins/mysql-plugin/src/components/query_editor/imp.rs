use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::sync::Arc;

use silo_plugin::ApplicationMessage;

#[derive(Debug, Default)]
pub struct MySQLQueryEditor {
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,
}

impl MySQLQueryEditor {}

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
