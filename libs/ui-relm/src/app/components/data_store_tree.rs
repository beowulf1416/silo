use tracing::debug;

use gtk::{Widget, gio, glib, prelude::*};
use std::cell::Ref;

use relm4::{
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
    // actions::{AccelsPlus, RelmAction, RelmActionGroup},
    // gtk,
    prelude::*,
};

use crate::app::components::data_store_view::DataStoreViewInputMsg;
use crate::app::models::{
    schema::Schema,
    store::{Store, StoreObjectType},
};

#[derive(Debug, Clone)]
enum Node {
    Store(Store),
    StoreObjectType(StoreObjectType),
    Schema(Schema),
}

impl Node {
    fn display_name(&self) -> &str {
        match self {
            Node::Store(store) => &store.name,
            Node::StoreObjectType(object) => &object.name,
            Node::Schema(schema) => &schema.name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataStoreTreeInputMsg {
    AddStore(Store),
    RemoveStore(Store),
}

#[derive(Debug)]
pub struct DataStoreTreeWidgets {
    tv: gtk::ColumnView,
}

#[derive(Debug, Clone)]
pub struct DataStoreTree {
    // pub stores: Vec<Store>,
}

// #[relm4::component(pub)]
impl SimpleComponent for DataStoreTree {
    type Init = ();
    type Input = DataStoreTreeInputMsg;
    type Output = DataStoreViewInputMsg;
    type Widgets = DataStoreTreeWidgets;
    type Root = gtk::Box;

    fn init_root() -> Self::Root {
        return gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let sample_stores = vec![
            Store {
                name: String::from("store 1"),
                object_types: vec![
                    StoreObjectType {
                        name: String::from("Schemas"),
                        object_types: vec![
                            StoreObjectType {
                                name: String::from("dbo"),
                                object_types: vec![],
                            },
                            StoreObjectType {
                                name: String::from("public"),
                                object_types: vec![],
                            },
                        ],
                    },
                    StoreObjectType {
                        name: String::from("Security"),
                        object_types: vec![
                            StoreObjectType {
                                name: String::from("Users"),
                                object_types: vec![],
                            },
                            StoreObjectType {
                                name: String::from("Roles"),
                                object_types: vec![],
                            },
                        ],
                    },
                ],
            },
            Store {
                name: String::from("store 2"),
                object_types: vec![
                    StoreObjectType {
                        name: String::from("Schemas"),
                        object_types: vec![
                            StoreObjectType {
                                name: String::from("dbo"),
                                object_types: vec![],
                            },
                            StoreObjectType {
                                name: String::from("public"),
                                object_types: vec![],
                            },
                        ],
                    },
                    StoreObjectType {
                        name: String::from("Security"),
                        object_types: vec![
                            StoreObjectType {
                                name: String::from("Users"),
                                object_types: vec![],
                            },
                            StoreObjectType {
                                name: String::from("Roles"),
                                object_types: vec![],
                            },
                        ],
                    },
                ],
            },
        ];

        let lstore = gio::ListStore::new::<glib::BoxedAnyObject>();
        for store in sample_stores {
            lstore.append(&glib::BoxedAnyObject::new(Node::Store(store)));
        }

        let model = gtk::TreeListModel::new(lstore, false, false, |item| {
            let node = item.downcast_ref::<glib::BoxedAnyObject>().unwrap();
            let node: Ref<Node> = node.borrow();

            // let store = match &*node {
            //     Node::Store(store) => store,
            //     Node::StoreObjectType(_) => return None,
            //     Node::Schema(_) => return None,
            // };

            // let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
            // for object in &store.objects {
            //     child_store.append(&glib::BoxedAnyObject::new(Node::StoreObjectType(
            //         object.clone(),
            //     )));
            // }

            let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
            match &*node {
                Node::Store(store) => {
                    for object in &store.object_types {
                        child_store.append(&glib::BoxedAnyObject::new(Node::StoreObjectType(
                            object.clone(),
                        )));
                    }
                }
                Node::StoreObjectType(ot) => {
                    for o in &ot.object_types {
                        child_store
                            .append(&glib::BoxedAnyObject::new(Node::StoreObjectType(o.clone())));
                    }
                }
                // todo
                Node::Schema(_) => {}
            }

            return Some(child_store.upcast());
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();
        let tv = gtk::ColumnView::builder().model(&selection).build();

        let name_factory = gtk::SignalListItemFactory::new();
        name_factory.connect_setup(|_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = gtk::TreeExpander::new();
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            expander.set_child(Some(&label));
            item.set_child(Some(&expander));
        });

        name_factory.connect_bind(|_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row: gtk::TreeListRow = item.property("item");

            if let Some(inner) = row.item() {
                let node: Ref<Node> = inner
                    .downcast_ref::<glib::BoxedAnyObject>()
                    .unwrap()
                    .borrow();

                let name = node.display_name().to_string();
                if let Some(expander) = item.child().and_downcast::<gtk::TreeExpander>() {
                    if let Some(child) = expander.child().and_downcast::<gtk::Label>() {
                        child.set_label(&name);
                    }
                    expander.set_list_row(Some(&row));
                }
            }
        });

        let name_column = gtk::ColumnViewColumn::builder()
            .title("Name")
            .factory(&name_factory)
            .build();
        tv.append_column(&name_column);

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();
        sw.set_child(Some(&tv));
        root.append(&sw);

        let model = Self {
            // stores: sample_stores.clone(),
        };
        let widgets = DataStoreTreeWidgets { tv };

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            DataStoreTreeInputMsg::AddStore(_store) => {
                debug!("//todo AddStore");
            }
            DataStoreTreeInputMsg::RemoveStore(_store) => {
                debug!("//todo RemoveStore");
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo");
    }
}
