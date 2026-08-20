use tracing::{debug, error};

use async_channel::Sender;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::plugins::PluginRegistry;

#[derive(Debug, Default)]
pub struct DataSourceView {
    pub registry: RefCell<Option<PluginRegistry>>,
}

impl DataSourceView {
    // pub fn set_plugin_registry(&mut self, registry: Rc<PluginRegistry>) {
    //     self.registry = registry;
    // }
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

        let menu_ds = gio::Menu::new();

        let menu_item =
            gio::MenuItem::new(Some("PostgreSQL"), Some("win.data-source-add::postgres"));
        menu_ds.append_item(&menu_item);

        let menu_item = gio::MenuItem::new(Some("MSSQL"), Some("win.data-source-add::mssql"));
        menu_ds.append_item(&menu_item);

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

        // let btn_add = gtk::Button::builder()
        //     .icon_name("list-add-symbolic")
        //     .tooltip_text("Add data source")
        //     .css_classes(vec!["btn", "flat"])
        //     .action_name("win.data-source-add")
        //     .build();

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

        // let sw = gtk::ScrolledWindow

        obj.append(&top_box);
    }
}

impl WidgetImpl for DataSourceView {}

impl BoxImpl for DataSourceView {}
