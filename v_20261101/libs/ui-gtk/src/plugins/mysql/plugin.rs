use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::{
    components::main_window::MainWindow,
    plugins::mysql::{connection_editor::MySQLConnectionEditor, query_editor::MySQLQueryEditor},
};

use crate::plugins::Plugin;

pub const PLUGIN_NAME: &str = "mysql";

pub fn factory() -> Box<dyn Plugin> {
    Box::new(MySQLPlugin::new())
}

#[derive(Debug)]
pub struct MySQLPlugin {}

impl MySQLPlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Plugin for MySQLPlugin {
    fn name(&self) -> &str {
        return PLUGIN_NAME;
    }

    fn build_data_source_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let editor = MySQLConnectionEditor::default();
        editor.set_main_window(window);
        return Some(editor.upcast());
    }

    fn build_query_editor_widget(&self, window: &MainWindow) -> Option<gtk::Widget> {
        let editor = MySQLQueryEditor::default();
        editor.set_main_window(window);
        return Some(editor.upcast());
    }
}
