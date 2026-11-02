use tracing::debug;

use gtk::{gio, glib, prelude::*, Widget};
use std::cell::Ref;
use std::fmt::Debug;
use std::rc::Rc;

use relm4::{
    // actions::{AccelsPlus, RelmAction, RelmActionGroup},
    // gtk,
    prelude::*,
    Component,
    ComponentParts,
    ComponentSender,
    SimpleComponent,
};

use crate::app::components::data_source_view::gnode::GNode;
use crate::app::components::data_source_view::node::{Node, SimpleNode};
use crate::app::components::data_source_view::DataSourceViewInputMsg;

#[derive(Debug)]
pub enum DataSourceTreeInputMsg {
    AddDataSource(Box<dyn Node>),
}

#[derive(Debug)]
pub struct DataSourceTreeWidgets {
    tv: gtk::ColumnView,
}

#[derive(Debug, Clone)]
pub struct DataSourceTree {}

impl DataSourceTree {
    fn build_stores() -> Vec<Rc<dyn Node>> {
        let stores: Vec<Rc<dyn Node>> = vec![
            Rc::new(SimpleNode {
                name: "store_1".to_string(),
                children: vec![
                    Rc::new(SimpleNode {
                        name: "Schemas".to_string(),
                        children: vec![],
                    }),
                    Rc::new(SimpleNode {
                        name: "Programmability".to_string(),
                        children: vec![],
                    }),
                    Rc::new(SimpleNode {
                        name: "Storage".to_string(),
                        children: vec![],
                    }),
                ],
            }),
            Rc::new(SimpleNode {
                name: "store_2".to_string(),
                children: vec![
                    Rc::new(SimpleNode {
                        name: "Schemas".to_string(),
                        children: vec![],
                    }),
                    Rc::new(SimpleNode {
                        name: "Programmability".to_string(),
                        children: vec![],
                    }),
                    Rc::new(SimpleNode {
                        name: "Storage".to_string(),
                        children: vec![],
                    }),
                ],
            }),
        ];

        return stores;
    }
}

// #[relm4::component(pub)]
impl SimpleComponent for DataSourceTree {
    type Init = ();
    type Input = DataSourceTreeInputMsg;
    type Output = DataSourceViewInputMsg;
    type Widgets = DataSourceTreeWidgets;
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
        let stores = Self::build_stores();

        // root level nodes
        let lstore = gio::ListStore::new::<GNode>();
        for store in stores {
            lstore.append(&GNode::new(store));
        }

        let model = gtk::TreeListModel::new(lstore, false, false, |item| {
            let gnode = item.downcast_ref::<GNode>().expect("GNode");
            let child_store = gio::ListStore::new::<GNode>();

            for child in gnode.node().children() {
                child_store.append(&GNode::new(child));
            }

            return Some(child_store.upcast());
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();
        let tv = gtk::ColumnView::builder().model(&selection).build();
        tv.append_column(&Self::build_name_column());

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();
        sw.set_child(Some(&tv));
        root.append(&sw);

        let model = Self {};
        let widgets = DataSourceTreeWidgets { tv };

        return ComponentParts { model, widgets };
    }
}
