pub mod mssql_connection_editor;

use gtk::{Box, glib, prelude::*};
use relm4::{Component, ComponentController, ComponentSender, prelude::*};

use crate::app::models::data_source::DataSourcePlugin;
use crate::app::windows::main::MainWindow;

#[derive(Debug, Clone)]
pub struct MssqlDataSourcePlugin {}

impl MssqlDataSourcePlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl DataSourcePlugin for MssqlDataSourcePlugin {
    fn name(&self) -> String {
        return "mssql".to_string();
    }

    fn build_editor_widget(
        &self,
    ) -> relm4::ComponentBuilder<mssql_connection_editor::MSSqlConnectionEditor> {
        // let widget = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .spacing(10)
        //     // .child(gtk::Label::builder().label("PostgreSQL").build())
        //     .build();

        // let label = gtk::Label::builder().label("PostgreSQL").build();
        // widget.append(&label);
        //
        //
        // let sender = sender.sender();

        let component = mssql_connection_editor::MSSqlConnectionEditor::builder()
            // .launch(())
            // .forward(sender.input_sender(), std::convert::identity);
            ;

        // return component.widget().clone().upcast::<gtk::Widget>();
        return cb;
    }
}
