use tracing::debug;

use gtk::{Widget, gio, glib, prelude::*};
use std::cell::Ref;

use relm4::{
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    // gtk,
    prelude::*,
};

// use crate::app::windows::main::DataSourceAddPostgresAction;
use crate::app::actions::DataSourceAddPostgresAction;
// use crate::app::actions::DataStoreAddAction;
use crate::app::windows::main::MainWindowMsg;

#[derive(Debug)]
enum Node {
    Store(Store),
    Object(String),
}

impl Node {
    fn display_name(&self) -> &str {
        match self {
            Node::Store(store) => &store.name,
            Node::Object(name) => name,
        }
    }
}

#[derive(Debug)]
struct Store {
    pub name: String,
    pub objects: Vec<String>,
}

#[derive(Debug)]
pub enum DataStoreViewInputMsg {
    NewDataStore,
}

// #[derive(Debug)]
// pub struct DataStoreViewWidgets {
//     // tv: gtk::ColumnView,
// }

#[derive(Debug, Clone)]
pub struct DataStoreView {
    // pub stores: Vec<Store>,
}

#[relm4::component(pub)]
impl SimpleComponent for DataStoreView {
    type Init = ();
    type Input = DataStoreViewInputMsg;
    type Output = MainWindowMsg;
    type Widgets = DataStoreViewWidgets;
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

            gtk::ActionBar {
                set_hexpand: true,

                pack_start = &gtk::MenuButton {
                    set_icon_name: "open-menu-symbolic",
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
        // let stores = vec![
        //     Store {
        //         name: "Store 1".to_string(),
        //         objects: vec![
        //             "Schemas".to_string(),
        //             "Tables".to_string(),
        //             "Views".to_string(),
        //         ],
        //     },
        //     Store {
        //         name: "Store 2".to_string(),
        //         objects: vec![
        //             "Schemas".to_string(),
        //             "Tables".to_string(),
        //             "Views".to_string(),
        //         ],
        //     },
        //     Store {
        //         name: "Store 3".to_string(),
        //         objects: vec![
        //             "Schemas".to_string(),
        //             "Tables".to_string(),
        //             "Views".to_string(),
        //         ],
        //     },
        //     Store {
        //         name: "Store 4".to_string(),
        //         objects: vec![
        //             "Schemas".to_string(),
        //             "Tables".to_string(),
        //             "Views".to_string(),
        //         ],
        //     },
        // ];

        // let lstore = gio::ListStore::new::<glib::BoxedAnyObject>();
        // for store in stores {
        //     lstore.append(&glib::BoxedAnyObject::new(Node::Store(store)));
        // }

        // let model = gtk::TreeListModel::new(lstore, false, false, |item| {
        //     let node = item.downcast_ref::<glib::BoxedAnyObject>().unwrap();
        //     let node: Ref<Node> = node.borrow();

        //     let store = match &*node {
        //         Node::Store(store) => store,
        //         Node::Object(_) => return None,
        //     };

        //     let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
        //     for object in &store.objects {
        //         child_store.append(&glib::BoxedAnyObject::new(Node::Object(object.clone())));
        //     }
        //     return Some(child_store.upcast());
        // });

        // let selection = gtk::SingleSelection::builder().model(&model).build();
        // let tv = gtk::ColumnView::builder().model(&selection).build();

        // let name_factory = gtk::SignalListItemFactory::new();
        // name_factory.connect_setup(|_factory, item| {
        //     let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        //     let expander = gtk::TreeExpander::new();
        //     let label = gtk::Label::new(None);
        //     label.set_xalign(0.0);
        //     expander.set_child(Some(&label));
        //     item.set_child(Some(&expander));
        // });

        // name_factory.connect_bind(|_factory, item| {
        //     let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        //     let row: gtk::TreeListRow = item.property("item");

        //     if let Some(inner) = row.item() {
        //         let node: Ref<Node> = inner
        //             .downcast_ref::<glib::BoxedAnyObject>()
        //             .unwrap()
        //             .borrow();

        //         let name = node.display_name().to_string();
        //         if let Some(expander) = item.child().and_downcast::<gtk::TreeExpander>() {
        //             if let Some(child) = expander.child().and_downcast::<gtk::Label>() {
        //                 child.set_label(&name);
        //             }
        //             expander.set_list_row(Some(&row));
        //         }
        //     }
        // });

        // let name_column = gtk::ColumnViewColumn::builder()
        //     .title("Name")
        //     .factory(&name_factory)
        //     .build();
        // tv.append_column(&name_column);

        // let sw = gtk::ScrolledWindow::builder()
        //     .hexpand(true)
        //     .vexpand(true)
        //     .build();
        // sw.set_child(Some(&tv));
        // root.append(&sw);

        // let model = Self {};
        // // let widgets = view_output!();
        // let widgets = DataStoreViewWidgets { tv };

        // return ComponentParts { model, widgets };

        let model = Self {};
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            DataStoreViewInputMsg::NewDataStore => {
                debug!("//todo NewDataStore");
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo");
    }
}
