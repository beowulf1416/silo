use tracing::{debug, error};

use async_channel::Sender;
use std::cell::RefCell;
// use std::rc::Rc;
use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use super::node::Node;
use crate::{
    components::data_source_view::{gnode, node::SimpleNode},
    plugins::PluginRegistry,
};
use gnode::GNode;

#[derive(Debug, Default)]
pub struct DataSourceView {
    pub registry: RefCell<Option<PluginRegistry>>,

    pub tv: gtk::ColumnView,
    pub sources: RefCell<Option<Vec<Arc<dyn Node>>>>,
}

impl DataSourceView {
    pub fn set_sources(&self, _sources: Vec<Arc<dyn Node>>) {
        // todo testing
        let sources: Vec<Arc<dyn Node>> = vec![
            Arc::new(SimpleNode {
                name: "source_1".to_string(),
                children: vec![
                    Arc::new(SimpleNode {
                        name: "Schemas".to_string(),
                        children: vec![],
                    }),
                    Arc::new(SimpleNode {
                        name: "Security".to_string(),
                        children: vec![
                            Arc::new(SimpleNode {
                                name: "Users".to_string(),
                                children: vec![],
                            }),
                            Arc::new(SimpleNode {
                                name: "Roles".to_string(),
                                children: vec![],
                            }),
                        ],
                    }),
                ],
            }),
            Arc::new(SimpleNode {
                name: "source_2".to_string(),
                children: vec![
                    Arc::new(SimpleNode {
                        name: "Schemas".to_string(),
                        children: vec![],
                    }),
                    Arc::new(SimpleNode {
                        name: "Security".to_string(),
                        children: vec![
                            Arc::new(SimpleNode {
                                name: "Users".to_string(),
                                children: vec![],
                            }),
                            Arc::new(SimpleNode {
                                name: "Roles".to_string(),
                                children: vec![],
                            }),
                        ],
                    }),
                ],
            }),
        ];

        self.sources.replace(Some(sources));
    }

    pub fn data_source_add(&self, node: Arc<dyn Node>) {
        debug!("data source add");
        self.sources
            .borrow_mut()
            // .get_or_insert(Vec::<Arc<dyn Node>>::new)
            .as_mut()
            .expect("Vec<Arc<dyn Node>> expected")
            .push(node);
    }

    pub fn build_name_column(&self) -> gtk::ColumnViewColumn {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_factory, item| {
            let litem = item.downcast_ref::<gtk::ListItem>().unwrap();

            let label = gtk::Label::new(None);
            label.set_xalign(0.0);

            let expander = gtk::TreeExpander::new();
            expander.set_child(Some(&label));

            litem.set_child(Some(&expander));
        });

        factory.connect_bind(|_factory, item| {
            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("expecting gtk::ListItem");
            let row = litem
                .item()
                .and_downcast::<gtk::TreeListRow>()
                .expect("expecting gtk::TreeListRow");

            let gnode = row.item().and_downcast::<GNode>().expect("expecting GNode");

            let expander = litem
                .child()
                .and_downcast::<gtk::TreeExpander>()
                .expect("expecting gtk::TreeExpander");
            expander.set_list_row(Some(&row));

            let label = expander
                .child()
                .and_downcast::<gtk::Label>()
                .expect("expecting gtk::Label");
            label.set_label(&gnode.node().display_name());
        });

        let column = gtk::ColumnViewColumn::new(Some("Name"), Some(factory));
        column.set_expand(true);
        return column;
    }

    pub fn build_tree(&self) {
        // let sources: std::vec::Vec<Rc<dyn Node>> = vec![];
        let sources = self
            .sources
            .borrow()
            .as_ref()
            .expect("Vec<Arc<dyn Node>> expected")
            .clone();

        // root level nodes
        let ls = gio::ListStore::new::<GNode>();
        for s in sources {
            ls.append(&GNode::new(Arc::clone(&s)));
        }

        let model = gtk::TreeListModel::new(ls, false, false, |item| {
            let gnode = item.downcast_ref::<GNode>().expect("GNode");
            let child_store = gio::ListStore::new::<GNode>();

            for child in gnode.node().children() {
                child_store.append(&GNode::new(Arc::clone(&child)));
            }

            return Some(child_store.upcast());
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();

        // let tv = gtk::ColumnView::builder().model(&selection).build();
        self.tv.set_model(Some(&selection));
        self.tv.append_column(&self.build_name_column());

        // let sw = gtk::ScrolledWindow::builder()
        //     .hexpand(true)
        //     .vexpand(true)
        //     .build();
        // sw.set_child(Some(&tv));
        // root.append(&sw);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for DataSourceView {
    const NAME: &'static str = "DataSourceView";
    type Type = super::DataSourceView;
    type ParentType = gtk::Box;
}

impl ObjectImpl for DataSourceView {
    fn constructed(&self) {
        debug!("DataSourceView::constructed");

        self.parent_constructed();
        let obj = self.obj();

        obj.set_hexpand(true);
        obj.set_vexpand(true);

        // menu
        let menu_ds = gio::Menu::new();

        let menu_item = gio::MenuItem::new(Some("Text"), Some("win.data-source-add::text"));
        menu_ds.append_item(&menu_item);

        let menu_section = gio::Menu::new();

        let menu_item =
            gio::MenuItem::new(Some("PostgreSQL"), Some("win.data-source-add::postgres"));
        menu_section.append_item(&menu_item);

        let menu_item = gio::MenuItem::new(Some("MSSQL"), Some("win.data-source-add::mssql"));
        menu_section.append_item(&menu_item);

        menu_ds.insert_section(1, None, &menu_section);

        // todo: empty at this point
        // debug!("registry {:?}", self.registry);
        // if let Some(registry) = self.registry.borrow().as_ref() {
        //     registry.registered_plugins().iter().for_each(|item| {
        //         debug!("item {:?}", item);
        //     });
        // }

        let top_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let header = gtk::Label::builder()
            .label("Data Sources")
            .halign(gtk::Align::Start)
            .margin_start(5)
            .margin_top(8)
            .build();
        top_box.append(&header);

        let btn_add = gtk::MenuButton::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("Add data source")
            .css_classes(vec!["btn", "flat"])
            .menu_model(&menu_ds)
            .build();

        let btn_remove = gtk::Button::builder()
            .icon_name("list-remove-symbolic")
            .tooltip_text("Remove data source")
            .css_classes(vec!["btn", "flat"])
            .action_name("win.data-source-remove")
            .build();

        let action_bar = gtk::ActionBar::builder()
            .tooltip_text("Data Source Actions")
            .hexpand(true)
            .build();

        action_bar.pack_start(&btn_add);
        action_bar.pack_start(&btn_remove);
        top_box.append(&action_bar);

        let todo: Vec<Arc<dyn Node>> = vec![];
        self.set_sources(todo);
        self.build_tree();

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&self.tv)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.append(&top_box);
        container.append(&sw);

        // obj.append(&top_box);
        // obj.append(&self.tv);
        obj.append(&container);
    }
}

impl WidgetImpl for DataSourceView {}

impl BoxImpl for DataSourceView {}
