use relm4::{ComponentSender, Worker, prelude::*};

pub trait DataSourcePlugin: std::fmt::Debug {
    fn name(&self) -> String;

    fn build_editor_widget<C: Component>(
        &self,
        // sender: ComponentSender<C>,
        // parent: gtk::Notebook,
    ) -> relm4::ComponentBuilder<C>;
}
