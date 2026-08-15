pub mod postgres_connection_editor;

use gtk::{Box, glib, prelude::*};
use relm4::{Component, ComponentController, ComponentSender, prelude::*};

use crate::app::models::data_source::DataSourcePlugin;

#[derive(Debug)]
pub struct PostgresDataSourcePlugin {}

impl PostgresDataSourcePlugin {
    pub fn new() -> Self {
        return Self {};
    }
}

impl DataSourcePlugin for PostgresDataSourcePlugin {
    fn name(&self) -> String {
        return "PostgreSQL".to_string();
    }

    fn build_editor_widget(&self, sender: ComponentSender<Self>) -> gtk::Widget {
        // let widget = gtk::Box::builder()
        //     .orientation(gtk::Orientation::Vertical)
        //     .spacing(10)
        //     // .child(gtk::Label::builder().label("PostgreSQL").build())
        //     .build();

        // let label = gtk::Label::builder().label("PostgreSQL").build();
        // widget.append(&label);

        let component = postgres_connection_editor::PostgresConnectionEditor::builder()
            .launch(())
            .forward(sender.input_sender(), std::convert::identity);

        return component.widget().upcast::<gtk::Widget>();
    }
}
