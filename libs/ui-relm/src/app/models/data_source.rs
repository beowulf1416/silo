use relm4::{ComponentSender, Worker, prelude::*};

pub trait DataSourcePlugin: std::fmt::Debug {
    fn name(&self) -> String;
    // fn build_editor_widget(&self) -> gtk::Widget;
    fn build_editor_widget(&self, sender: ComponentSender<Self>) -> gtk::Widget;
}
