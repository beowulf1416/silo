use tracing::debug;

use std::cell::Ref;
use std::convert::identity;

use gtk::{ColumnView, gio, glib, prelude::*};
use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent, prelude::*};

use crate::app::components::data_store::DataStoreMsg;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum TreeMsg {
    EXPAND,
    COLLAPSE,
}

#[derive(Debug)]
pub struct Tree {
    sw: gtk::ScrolledWindow,
    tree: gtk::ColumnView,
}

// // #[derive(Debug)]
// pub struct TreeWidgets {
//     tree: gtk::ColumnView,
// }

#[relm4::component(pub)]
impl SimpleComponent for Tree {
    type Input = TreeMsg;
    type Output = DataStoreMsg;
    type Init = ();
    // type Widgets = TreeWidgets;
    // type Root = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "test"
            },

            append: &model.sw
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
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        debug!("init");
        // let model = Self {};
        // let widgets = view_output!();

        // build tree column view with 1 column "Name"

        // let nodes = Vec::new();
        let nodes = vec![
            Node {
                name: "test1".to_string(),
                children: Vec::new(),
            },
            Node {
                name: "test2".to_string(),
                children: Vec::new(),
            },
            Node {
                name: "test3".to_string(),
                children: Vec::new(),
            },
        ];

        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        for node in nodes {
            store.append(&glib::BoxedAnyObject::new(node));
        }

        let model = gtk::TreeListModel::new(store, false, false, |item| {
            let node = item.downcast_ref::<glib::BoxedAnyObject>().unwrap();
            let node: Ref<Node> = node.borrow();

            let child_store = gio::ListStore::new::<glib::BoxedAnyObject>();
            for child in node.children.clone() {
                child_store.append(&glib::BoxedAnyObject::new(child));
            }
            return Some(child_store.upcast());
        });
        let selection = gtk::SingleSelection::builder().model(&model).build();
        let tv = gtk::ColumnView::builder().model(&selection).build();
        tv.set_vexpand(true);
        tv.set_hexpand(true);

        let name_factory = gtk::SignalListItemFactory::new();
        name_factory.connect_setup(|_factory, item| {
            debug!("{:?}", item);
            // let node = item.downcast_ref::<glib::BoxedAnyObject>().unwrap();
            // let node: Ref<Node> = node.borrow();

            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = gtk::TreeExpander::new();
            let label = gtk::Label::new(None);
            // let label = gtk::Label::new(Some(&node.name));
            label.set_xalign(0.0);
            expander.set_child(Some(&label));
            item.set_child(Some(&expander));
        });

        // name_factory.connect_bind(|_factory, item| {
        //     let label = item.child().and_downcast().unwrap();
        // });

        let name_column = gtk::ColumnViewColumn::builder()
            .factory(&name_factory)
            .title("Name")
            .build();
        tv.append_column(&name_column);

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();
        sw.set_child(Some(&tv));

        let model = Self { sw: sw, tree: tv };
        // let widgets = TreeWidgets { tree: tv };
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        // match message {
        //     MainWindowMsg::Quit => main_application().quit(),
        // }
        debug!("//todo");
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        // widgets.save_window_size().unwrap();
        debug!("//todo");
    }
}
