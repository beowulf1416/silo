use tracing::{debug, error};

use async_channel::Sender;
use std::cell::RefCell;
use std::sync::Arc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::components::data_sources_view::gnode::GNode;
use crate::components::data_sources_view::tree_node::*;

#[derive(Debug)]
pub struct DataSourcesView {
    pub store: gio::ListStore,
    pub tv: gtk::ColumnView,
    pub sources: Vec<Arc<data_source_node::DataSourceNode>>,
}

impl DataSourcesView {
    pub fn new() -> Self {
        return Self {
            store: gio::ListStore::new::<TreeNode>(),
            sources: vec![],
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
        // todo
        let dsn = data_source_node::DataSourceNode::new(
            &"test_name",
            &"test_host",
            &1234u16,
            &"test_user",
            &"test_pw",
            &"test_db",
        );
        let node = Node::DataSourceNode(dsn);
        self.store.append(&TreeNode::new(node));

        let model = gtk::TreeListModel::new(self.store.clone(), false, false, |obj| {
            let node = obj.downcast_ref::<TreeNode>().expect("//todo TreeNode");
            return Self::create_child_model(&node);
        });

        let selection = gtk::SingleSelection::builder().model(&model).build();

        self.tv.set_model(Some(&selection));
        self.tv.append_column(&self.build_name_column());

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&self.tv)
            .build();

        return sw;
    }

    fn create_child_model(tn: &TreeNode) -> Option<gio::ListModel> {
        debug!("//todo: create_child_model {:?}", tn);

        let store = gio::ListStore::new::<TreeNode>();

        match tn.node() {
            Node::DataSourceNode(dsn) => {
                debug!("DataSourceNode {:?}", dsn);

                store.append(&TreeNode::new(Node::SchemaNode(
                    schema_node::SchemaNode::new("public"),
                )));
                store.append(&TreeNode::new(Node::SchemaNode(
                    schema_node::SchemaNode::new("eas"),
                )));
            }
            Node::SchemaNode(sn) => {
                debug!("//todo SchemaNode");
                store.append(&TreeNode::new(Node::SchemaObjectNode(
                    schema_object_node::SchemaObjectNode::new("Tables"),
                )));
                store.append(&TreeNode::new(Node::SchemaObjectNode(
                    schema_object_node::SchemaObjectNode::new("Stored Proceduers"),
                )));
            }
            Node::SchemaObjectNode(son) => {
                debug!("//todo SchemaObjectNode");
            }
            Node::TableNode => {
                debug!("//todo TableNode");
            }
        }

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
            row.connect_expanded_notify(|row| {
                let is_expanded = row.is_expanded();
                debug!("row is expanded {}", is_expanded);
            });

            let node = row
                .item()
                .and_downcast::<TreeNode>()
                .expect("expecting TreeNode");

            let expander = litem
                .child()
                .and_downcast::<gtk::TreeExpander>()
                .expect("expecting gtk::TreeExpander");
            expander.set_list_row(Some(&row));

            let label = expander
                .child()
                .and_downcast::<gtk::Label>()
                .expect("expecting gtk::Label");
            label.set_label(&node.name());
        });

        let column = gtk::ColumnViewColumn::new(Some("Name"), Some(factory));
        column.set_expand(true);
        return column;
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
