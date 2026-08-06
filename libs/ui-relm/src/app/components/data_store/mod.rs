pub mod tree;

use std::convert::identity;
use tracing::debug;

use gtk::prelude::{
    ActionableExt, ApplicationExt, BoxExt, ButtonExt, GtkWindowExt, OrientableExt, SettingsExt,
    WidgetExt,
};
use gtk::{gio, glib};

use relm4::{
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    // actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk,
    prelude::*,
};

// use crate::app::main_window::MainWindowMsg;
use crate::app::components::data_store::tree::Tree;
use crate::app::windows::main::MainWindowMsg;

#[derive(Debug)]
pub enum DataStoreMsg {
    TODO,
}

#[derive(Debug)]
pub struct DataStoreWindow {
    pub tree: Controller<Tree>,
}

// pub struct DataStoreWidgets {

// }

#[relm4::component(pub)]
impl SimpleComponent for DataStoreWindow {
    type Init = ();
    type Input = DataStoreMsg;
    type Output = MainWindowMsg;
    type Widgets = DataStoreWidgets;
    // type Root = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::ActionBar {
                set_hexpand: true,

                pack_start = &gtk::Button {
                    set_label: "Open",
                    set_icon_name: "document-open",
                    set_action_name: Some("win.database-new")
                },
                pack_start = &gtk::Button {
                    set_label: "Save",
                    set_icon_name: "document-save",
                    set_action_name: Some("win.database-new")
                }
            },

            model.tree.widget(),
        }
    }

    // fn init_root() -> Self::Root {
    //     return gtk::Box::builder()
    //         .orientation(gtk::Orientation::Vertical)
    //         .vexpand(true)
    //         .hexpand(true)
    //         .build();
    // }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tree = Tree::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = Self { tree: tree };
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            DataStoreMsg::TODO => {
                debug!("//todo");
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo");
    }
}

impl DataStoreWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        return Ok(());
    }

    fn load_window_size(&self) {
        debug!("//todo");
    }
}
