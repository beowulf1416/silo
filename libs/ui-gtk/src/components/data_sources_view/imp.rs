use tracing::{debug, error};

use async_channel::Sender;
use std::cell::{Ref, RefCell};
use std::result;
use std::sync::Arc;
use tracing_subscriber::field::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::components::data_sources_view::LoadingNode;
// use crate::lib::get_runtime;

use silo_plugin::node::Node;

#[derive(Debug)]
pub struct DataSourcesView {
    pub store: gio::ListStore,
    pub tv: gtk::ColumnView,
    // pub sources: Vec<Arc<data_source_node::DataSourceNode>>,
}

impl DataSourcesView {
    pub fn new() -> Self {
        return Self {
            // store: gio::ListStore::new::<TreeNode>(),
            store: gio::ListStore::new::<glib::BoxedAnyObject>(),
            // sources: vec![],
            tv: gtk::ColumnView::builder()
                .hexpand(true)
                .vexpand(true)
                .build(),
        };
    }

    fn build_menu(&self) -> gio::Menu {
        let menu = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Text"), Some("win.data-source-add::text"));
        menu.append_item(&item);

        let item = gio::MenuItem::new(Some("PostgreSQL"), Some("win.data-source-add::postgres"));
        let section = gio::Menu::new();
        section.append_item(&item);

        let item = gio::MenuItem::new(Some("MySQL"), Some("win.data-source-add::mysql"));
        section.append_item(&item);

        let item = gio::MenuItem::new(Some("MSSQL"), Some("win.data-source-add::mssql"));
        section.append_item(&item);

        menu.append_section(None, &section);

        return menu;
    }

    pub fn build_header(&self) -> gtk::Box {
        let icon = gtk::Image::builder()
            .icon_name("data_source")
            .halign(gtk::Align::Start)
            .margin_start(12)
            .margin_top(4)
            .build();
        let label = gtk::Label::builder().label("Data Sources").build();

        let top = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
        top.append(&icon);
        top.append(&label);

        return top;
    }

    pub fn build_action_bar(&self) -> gtk::ActionBar {
        let menu = self.build_menu();

        let btn_add = gtk::MenuButton::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("Add data source")
            .css_classes(vec!["btn", "flat"])
            .menu_model(&menu)
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

        return action_bar;
    }

    pub fn build_tree(&self) -> gtk::ScrolledWindow {
        // testing
        // let boxed: Box<dyn Node> = Box::new(PostgresDataSourceNode::new("testing"));
        // self.store.append(&glib::BoxedAnyObject::new(boxed));

        let model = gtk::TreeListModel::new(self.store.clone(), false, false, |obj| {
            let node = obj
                .downcast_ref::<glib::BoxedAnyObject>()
                .expect("//todo BoxedAnyObject");
            return Self::create_child_model(&node);

            // let store = gio::ListStore::new();

            // store.append(&glib::BoxedAnyObject::new(Box::new(LoadingNode {})));

            // return Some(store.upcast());
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();

        self.tv.set_model(Some(&selection));
        self.tv.append_column(&self.build_name_column());
        self.tv.append_column(&self.build_menu_column());

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&self.tv)
            .build();

        return sw;
    }

    fn create_child_model(obj: &glib::BoxedAnyObject) -> Option<gio::ListModel> {
        // let node_ref: Ref<Box<dyn Node>> = obj.borrow::<Box<dyn Node>>();
        // let node: &dyn Node = node_ref.as_ref();

        // if let Some(store) = node.children() {
        //     return Some(store.upcast());
        // } else {
        //     return None;
        // }

        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        store.append(&glib::BoxedAnyObject::new(Box::new(LoadingNode {})));

        let store_clone = store.clone();
        let obj_clone = obj.clone();
        // let node_clone = node.clone();

        let main_context = glib::MainContext::default();
        main_context.spawn_local(async move {
            let node_ref: Ref<Box<dyn Node>> = obj_clone.borrow::<Box<dyn Node>>();
            let node: &dyn Node = node_ref.as_ref();

            match node.children() {
                None => {
                    store_clone.remove_all();
                }
                Some(children) => {
                    store_clone.remove_all();
                    for child in children {
                        store_clone.append(&glib::BoxedAnyObject::new(Box::new(child)));
                    }
                }
            }
        });

        return Some(store.upcast());
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
            // row.connect_expanded_notify(|row| {
            //     let is_expanded = row.is_expanded();
            //     debug!("row is expanded {}", is_expanded);
            // });

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

            let node_ref: Ref<Box<dyn Node>> = obj.borrow::<Box<dyn Node>>();
            let node: &dyn Node = node_ref.as_ref();

            label.set_label(&node.name());
            // if let Some(menu) = node.context_menu() {
            //     label.set_extra_menu(Some(&menu));
            // }
        });

        let column = gtk::ColumnViewColumn::new(Some("Name"), Some(factory));
        column.set_expand(true);
        return column;
    }

    fn build_menu_column(&self) -> gtk::ColumnViewColumn {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_factory, item| {
            let litem = item.downcast_ref::<gtk::ListItem>().unwrap();

            let button = gtk::MenuButton::builder()
                .icon_name("ellipsis_vertical")
                .tooltip_text("Actions")
                .css_classes(vec!["btn", "flat"])
                .build();

            litem.set_child(Some(&button));
        });

        factory.connect_bind(|_factory, item| {
            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("expecting gtk::ListItem");

            let row = litem
                .item()
                .and_downcast::<gtk::TreeListRow>()
                .expect("expecting gtk::TreeListRow");

            let button = litem
                .child()
                .and_downcast::<gtk::MenuButton>()
                .expect("expecting gtk::MenuButton");

            let obj = row
                .item()
                .and_downcast::<glib::BoxedAnyObject>()
                .expect("expecting BoxedAnyObject");

            let node_ref: Ref<Box<dyn Node>> = obj.borrow::<Box<dyn Node>>();
            let node: &dyn Node = node_ref.as_ref();

            if let Some(menu) = node.context_menu() {
                let pop_menu = gtk::PopoverMenu::from_model(Some(&menu));
                button.set_popover(Some(&pop_menu));
            } else {
                button.set_active(false);
            }
        });

        let column = gtk::ColumnViewColumn::new(None, Some(factory));
        column.set_expand(false);
        return column;
    }

    pub fn data_source_add(&self, node: Box<dyn Node>) {
        debug!("data_source_add {:?}", node);
        self.store.append(&glib::BoxedAnyObject::new(node));
    }
}

impl Default for DataSourcesView {
    fn default() -> Self {
        return Self::new();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for DataSourcesView {
    const NAME: &'static str = "DataSourcesView";
    type Type = super::DataSourcesView;
    type ParentType = gtk::Box;
}

impl ObjectImpl for DataSourcesView {
    fn constructed(&self) {
        debug!("DataSourceView::constructed");

        self.parent_constructed();
        let obj = self.obj();
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        let header = self.build_header();
        container.append(&header);

        let action_bar = self.build_action_bar();
        container.append(&action_bar);

        let sw = self.build_tree();
        container.append(&sw);

        obj.append(&container);
    }
}

impl WidgetImpl for DataSourcesView {}

impl BoxImpl for DataSourcesView {}
