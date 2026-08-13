pub mod node;
pub mod gnode;
pub mod tree;


use tracing::debug;

use gtk::{
    Widget, gio, glib,
    prelude::{BoxExt, *},
};
use std::{cell::Ref, convert::identity};

use relm4::{
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    // gtk,
    prelude::*,
};

use crate::app::{
    actions::DataSourceAddPostgresAction,
    // components::data_store_tree::DataStoreTree,
    // components::data_source_view::tree::DataSourceTree,
};
use crate::app::windows::main::MainWindowMsg;
use crate::app::components::data_source_view::tree::DataSourceTree;


#[derive(Debug)]
pub enum DataSourceViewInputMsg {
    NewDataSource,
}


#[derive(Debug)]
pub struct DataSourceView {
    pub tree: Controller<DataSourceTree>,
}

#[relm4::component(pub)]
impl SimpleComponent for DataSourceView {
    type Init = ();
    type Input = DataSourceViewInputMsg;
    type Output = MainWindowMsg;
    type Widgets = DataSourceViewWidgets;
    // type Root = gtk::Box;

    menu! {
        data_store_add_menu: {
            section! {
                "PostgreSQL" => DataSourceAddPostgresAction,
                "MySQL" => DataSourceAddPostgresAction,
                "MSSQL" => DataSourceAddPostgresAction,
            }
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "Data Sources",
                set_align: gtk::Align::Start,
                set_margin_start: 4,
                set_margin_top: 4,
            },

            gtk::ActionBar {
                set_hexpand: true,

                pack_start = &gtk::MenuButton {
                    set_icon_name: "list-add-symbolic",
                    set_menu_model: Some(&data_store_add_menu),
                },

                pack_start = &gtk::MenuButton {
                    set_icon_name: "list-remove-symbolic",
                    set_menu_model: Some(&data_store_add_menu),
                },

                pack_start = &gtk::Button {
                    set_label: "Add",
                    set_icon_name: "document-open",
                    set_action_name: Some("win.data-store-add")
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

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let dst = DataSourceTree::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = Self {
            tree: dst,
        };
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            DataSourceViewInputMsg::NewDataSource => {
                debug!("//todo NewDataStore");
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo shutdown");
    }
}
