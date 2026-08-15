use tracing::debug;

use gtk::{gdk, gio, prelude::*};
use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent, prelude::*};

#[derive(Debug)]
pub enum PostgresConnectionEditorInputMsg {
    Connect,
}

#[derive(Debug)]
pub struct PostgresConnectionEditor {}

#[relm4::component(pub)]
impl SimpleComponent for PostgresConnectionEditor {
    type Init = ();
    type Input = PostgresConnectionEditorInputMsg;
    type Output = ();
    type Widgets = ComponentWidgets;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "postgres_test"
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        return ComponentParts {
            model: model,
            widgets: widgets,
        };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        debug!("postgres_connection_editor::update {:?}", message);
    }
}
