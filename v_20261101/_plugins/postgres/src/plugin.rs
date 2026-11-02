use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::ApplicationMessage;
use silo_plugin::app_window::AppWindow;
use silo_plugin::plugin::Plugin;

use crate::components::{
    connection_editor::PostgresConnectionEditor,
    // query_editor::PostgresQueryEditor,
};

pub fn factory(window: Arc<dyn AppWindow>) -> Arc<dyn Plugin> {
    let boxed: Arc<dyn Plugin> = Arc::new(PostgresPlugin::new(window));
    return boxed;
}

#[derive(Debug)]
pub struct PostgresPlugin {
    app: Arc<dyn AppWindow>,
}

impl PostgresPlugin {
    pub fn new(app: Arc<dyn AppWindow>) -> Self {
        return Self { app };
    }

    pub fn app(&self) -> &dyn AppWindow {
        return &*self.app;
    }
}

impl Plugin for PostgresPlugin {
    fn name(&self) -> &str {
        return "postgres";
    }

    fn build_data_source_editor_widget(
        &self,
        sender: async_channel::Sender<ApplicationMessage>,
    ) -> Option<gtk::Widget> {
        let editor = PostgresConnectionEditor::default();
        // editor.set_main_window(window);
        editor.set_sender(sender);
        return Some(editor.upcast());
    }

    // fn build_query_editor_widget(
    //     &self,
    //     sender: async_channel::Sender<ApplicationMessage>,
    // ) -> Option<gtk::Widget> {
    //     let editor = PostgresQueryEditor::default();
    //     editor.set_sender(sender);
    //     return Some(editor.upcast());
    // }
    //

    // fn get_pool(&self) -> Result<sqlx::Pool, &'static str> {
    //     return Err("//todo get_pool");
    // }
}
