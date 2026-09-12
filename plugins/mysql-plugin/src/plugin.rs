use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::ApplicationMessage;
use silo_plugin::app_window::AppWindow;
use silo_plugin::plugin::Plugin;

use crate::components::{
    connection_editor::MySQLConnectionEditor,
    // query_editor::MySQLQueryEditor
};

pub fn factory(window: Arc<dyn AppWindow>) -> Arc<dyn Plugin> {
    let boxed: Arc<dyn Plugin> = Arc::new(MySQLPlugin::new(window));
    return boxed;
}

#[derive(Debug)]
pub struct MySQLPlugin {
    app: Arc<dyn AppWindow>,
}

impl MySQLPlugin {
    pub fn new(app: Arc<dyn AppWindow>) -> Self {
        return Self { app };
    }
}

impl Plugin for MySQLPlugin {
    fn name(&self) -> &str {
        return "mysql";
    }

    fn build_data_source_editor_widget(
        &self,
        sender: async_channel::Sender<ApplicationMessage>,
    ) -> Option<gtk::Widget> {
        let editor = MySQLConnectionEditor::default();
        editor.set_sender(sender);
        return Some(editor.upcast());
    }

    // fn build_query_editor_widget(
    //     &self,
    //     sender: async_channel::Sender<ApplicationMessage>,
    // ) -> Option<gtk::Widget> {
    //     let editor = MySQLQueryEditor::default();
    //     editor.set_sender(sender);
    //     return Some(editor.upcast());
    // }

    // fn get_pool(&self) -> Result<sqlx::Pool, &'static str> {
    //     return Err("//todo get_pool");
    // }
}
