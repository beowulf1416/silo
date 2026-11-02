pub mod gnode;
pub mod node;
pub mod tree;

use tracing::debug;

use gtk::{
    Widget, gio, glib,
    prelude::{BoxExt, *},
};

use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::Ref, convert::identity};

use relm4::{
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    // actions::{AccelsPlus, RelmAction, RelmActionGroup},
    // gtk,
    prelude::*,
};

use crate::app::windows::main::MainWindowMsg;
use crate::app::{
    actions::NewDataSourcePostGresAction,
    // components::data_store_tree::DataStoreTree,
    // components::data_source_view::tree::DataSourceTree,
};
use crate::app::{
    components::data_source_view::tree::DataSourceTree, models::data_source::DataSourcePlugin,
};

type DataSourcePluginName = String;

#[derive(Debug)]
pub enum DataSourceViewInputMsg {
    NewDataSource(Rc<dyn DataSourcePlugin>),
}

#[derive(Debug, Clone)]
pub struct DataSourceInit {
    pub data_source_plugins: HashMap<String, Rc<dyn DataSourcePlugin>>,
}

#[derive(Debug)]
pub struct DataSourceView {
    pub tree: Controller<DataSourceTree>,
}

#[relm4::component(pub)]
impl SimpleComponent for DataSourceView {
    type Init = DataSourceInit;
    type Input = DataSourceViewInputMsg;
    type Output = MainWindowMsg;
    type Widgets = DataSourceViewWidgets;
    // type Root = gtk::Box;

    menu! {
        data_store_add_menu: {
            section! {
                "PostgreSQL" => NewDataSourcePostGresAction,
                "MySQL" => NewDataSourcePostGresAction,
                "MSSQL" => NewDataSourcePostGresAction,
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

        let model = Self { tree: dst };
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        // debug!("sender {:?}", sender);

        match message {
            DataSourceViewInputMsg::NewDataSource(plugin) => {
                debug!("//todo NewDataStore {}", plugin.name());

                // let widget = plugin.build_editor_widget(sender);
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo shutdown");
    }
}
