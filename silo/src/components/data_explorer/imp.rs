use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use std::cell::{Ref, RefCell};
use std::sync::Arc;
use tracing::{debug, error, info};

use silo_api::{
    Silo,
    node::{Node, NodeType},
};

use crate::app::App;
use crate::components::app_window::AppWindow;
use crate::components::data_explorer::node::NodeImpl;
use crate::get_runtime;

#[derive(Debug)]
pub struct DataExplorerImpl {
    pub silo: RefCell<Option<Arc<dyn Silo>>>,

    pub(super) window: RefCell<Option<AppWindow>>,
    pub(super) store: gio::ListStore,
}

impl DataExplorerImpl {
    pub fn build_header(&self) -> gtk::Box {
        let lbl = gtk::Label::builder()
            .label("Data Explorer")
            .tooltip_text("Data Explorer")
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .margin_start(10)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        container.append(&lbl);

        return container;
    }

    pub fn build_action_bar(&self) -> gtk::Box {
        let btn_add = gtk::Button::builder()
            .tooltip_text("Add Data Source")
            .icon_name("data-source-add-symbolic")
            .build();

        let btn_remove = gtk::Button::builder()
            .tooltip_text("Remove Data Source")
            .icon_name("data-source-remove-symbolic")
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .margin_start(4)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        container.append(&btn_add);
        container.append(&btn_remove);

        return container;
    }

    pub fn build_tree(&self) -> gtk::Box {
        let model = gtk::TreeListModel::new(self.store.clone(), false, false, move |obj| {
            let boxed = obj
                .downcast_ref::<glib::BoxedAnyObject>()
                .expect("//todo BoxedAnyObject");
            // return Self::create_child_model(&boxed);

            let store = gio::ListStore::new::<glib::BoxedAnyObject>();
            return Some(store.upcast());
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();

        let tv = gtk::ColumnView::builder()
            .hexpand(true)
            .vexpand(true)
            .model(&selection)
            .build();
        tv.append_column(&self.build_name_column());

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&tv)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .margin_start(4)
            .margin_end(4)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        container.append(&sw);

        return container;
    }

    pub fn create_child_nodes(&self, obj: &glib::BoxedAnyObject) -> Option<gio::ListModel> {
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        return Some(store.upcast());
    }

    pub fn set_silo(&self, silo: &Arc<dyn Silo>) {
        info!("DataExplorerImp::set_silo");

        let silo_clone = Arc::clone(&silo);

        self.silo.replace(Some(Arc::clone(silo)));

        let sources = silo_clone.sources();
        for s in sources {
            // let mut node: Arc<dyn Node> = Arc::new(NodeImpl::new(
            //     &s.name,
            //     NodeType::DataSource,
            //     &s.plugin,
            //     &s.data_source_name,
            // ));

            let plugin = silo.plugin_manager().get_plugin(&s.plugin);

            let mut node = NodeImpl::new(
                &s.name,
                NodeType::DataSource,
                plugin,
                &s.data_source_name,
                &s.data_source_name,
            );
            node.set_silo(Arc::clone(&silo_clone));
            let arc_dyn_node: Arc<dyn Node> = Arc::new(node);

            self.store.append(&glib::BoxedAnyObject::new(arc_dyn_node));
        }
    }

    pub fn build_name_column(&self) -> gtk::ColumnViewColumn {
        // let silo = self.silo.borrow().as_ref().expect("Silo not set");

        // let silo = {
        //     let s = self.silo.borrow();
        //     let s = s.as_ref().expect("Silo is not set");
        //     s.clone()
        // };

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_factory, item| {
            let litem = item.downcast_ref::<gtk::ListItem>().unwrap();

            let label = gtk::Label::new(None);
            label.set_xalign(0.0);

            let expander = gtk::TreeExpander::new();
            expander.set_child(Some(&label));

            litem.set_child(Some(&expander));
        });

        factory.connect_bind(move |_factory, item| {
            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("expecting gtk::ListItem");

            let row = litem
                .item()
                .and_downcast::<gtk::TreeListRow>()
                .expect("expecting gtk::TreeListRow");

            // let silo_clone = silo.clone();
            row.connect_notify(Some("expanded"), move |row, _| {
                if row.is_expanded() {
                    info!("row is expanded");

                    if let Some(model) = row.children() {
                        if let Ok(store) = model.downcast::<gio::ListStore>() {
                            debug!("store {:?}", store);

                            let obj = row
                                .item()
                                .and_downcast::<glib::BoxedAnyObject>()
                                .expect("expecting BoxedAnyObject");
                            // let node = obj.borrow::<Arc<dyn Node>>();
                            let node: Arc<dyn Node> = {
                                let node_ref: Ref<Arc<dyn Node>> = obj.borrow::<Arc<dyn Node>>();
                                Arc::clone(&node_ref)
                            };

                            // let silo = self
                            //     .silo
                            //     .borrow()
                            //     .as_ref()
                            //     .expect("unable to obtain silo object");

                            // let value = silo_clone.clone();
                            let handle = get_runtime().spawn(async move {
                                // let child_nodes = node.children(Arc::clone(silo)).await;
                                // child_nodes
                                let child_nodes = node.children().await;
                                child_nodes
                            });

                            glib::MainContext::default().spawn_local(async move {
                                match handle.await {
                                    Err(e) => {
                                        error!("error {:?}", e);
                                    }
                                    Ok(result) => {
                                        debug!("result {:?}", result);
                                        // store.remove_all();
                                    }
                                }
                            });
                        } else {
                            error!("unable to retrieve child list store");
                        }
                    } else {
                        error!("unable to retrieve child list model");
                    }
                } else {
                    info!("row is collapsed");
                }

                //                 let obj = row
                //                     .item()
                //                     .and_downcast::<glib::BoxedAnyObject>()
                //                     .expect("expecting BoxedAnyObject");
                //
                //                 let node = obj.borrow::<Arc<dyn Node>>();
                //                 debug!("node (connect_notify) {:?}", node);
            });

            let expander = litem
                .child()
                .and_downcast::<gtk::TreeExpander>()
                .expect("expecting gtk::TreeExpander");
            expander.set_list_row(Some(&row));

            let label = expander
                .child()
                .and_downcast::<gtk::Label>()
                .expect("expecting gtk::Label");

            let obj = row
                .item()
                .and_downcast::<glib::BoxedAnyObject>()
                .expect("expecting BoxedAnyObject");

            //             let node_ref: Ref<Arc<dyn Node>> = obj.borrow::<Arc<dyn Node>>();
            //             let node: &dyn Node = node_ref.as_ref();

            let node = obj.borrow::<Arc<dyn Node>>();
            debug!("node {:?}", node);
            label.set_label(&node.name());
        });

        let column = gtk::ColumnViewColumn::new(Some("Name"), Some(factory));
        column.set_expand(true);
        return column;
    }
}

impl Default for DataExplorerImpl {
    fn default() -> Self {
        return Self {
            silo: RefCell::new(None),
            window: RefCell::new(None),
            store: gio::ListStore::new::<glib::BoxedAnyObject>(),
        };
    }
}

#[glib::object_subclass]
impl ObjectSubclass for DataExplorerImpl {
    const NAME: &'static str = "DataExplorerImpl";
    type Type = super::DataExplorer;
    type ParentType = gtk::Box;
}

impl ObjectImpl for DataExplorerImpl {
    fn constructed(&self) {
        info!("DataExplorerImpl::constructed()");

        self.parent_constructed();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.append(&self.build_header());
        container.append(&self.build_action_bar());
        container.append(&self.build_tree());

        let obj = self.obj();
        obj.set_vexpand(true);
        obj.set_hexpand(true);
        obj.append(&container);
    }
}

impl WidgetImpl for DataExplorerImpl {}

impl BoxImpl for DataExplorerImpl {}
