pub mod postgres_connection_editor;

use gtk::{Box, glib, prelude::*};
use relm4::{Component, ComponentController, ComponentSender, prelude::*};

use crate::app::{
    components::data_source_view::DataSourceView, models::data_source::DataSourcePlugin,
};
// use crate::app::windows::main::MainWindow;

#[derive(Debug, Clone)]
pub struct PostgresDataSourcePlugin {}

impl PostgresDataSourcePlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl DataSourcePlugin for PostgresDataSourcePlugin {
    fn name(&self) -> String {
        return "postgresql".to_string();
    }

    fn build_editor_widget<PostgresConnectionEditor>(
        &self,
        // sender: ComponentSender<DataSourceView>,
        // parent: gtk::Notebook,
    ) -> relm4::ComponentBuilder<postgres_connection_editor::PostgresConnectionEditor> {
        // let sender = sender.sender();

        let cb = postgres_connection_editor::PostgresConnectionEditor::builder()
            // .launch(())
            // .forward(sender.input_sender(), std::convert::identity);
            ;

        // return component.widget().clone().upcast::<gtk::Widget>();
        return cb;
    }
}
