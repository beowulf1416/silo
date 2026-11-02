use tracing::debug;

use gtk::{gdk, gio, prelude::*};
use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent, prelude::*};

use crate::app::windows::main::MainWindowMsg;

#[derive(Debug)]
pub enum MSSqlConnectionEditorInputMsg {
    Connect,
}

#[derive(Debug)]
pub struct MSSqlConnectionEditor {}

#[relm4::component(pub)]
impl SimpleComponent for MSSqlConnectionEditor {
    type Init = ();
    type Input = MSSqlConnectionEditorInputMsg;
    type Output = MainWindowMsg;
    type Widgets = ComponentWidgets;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "mssql_test"
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
        debug!("mssql_connection_editor::update {:?}", message);
    }
}
