use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::ApplicationMessage;
use silo_plugin::plugin::Plugin;

use crate::components::{
    connection_editor::PostgresConnectionEditor,
    // query_editor::PostgresQueryEditor,
};

pub fn factory() -> Box<dyn Plugin> {
    let boxed: Box<dyn Plugin> = Box::new(PostgresPlugin::new());
    return boxed;
}

#[derive(Debug)]
pub struct PostgresPlugin {}

impl PostgresPlugin {
    pub fn new() -> Self {
        return Self {};
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
