use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use tracing::debug;

#[derive(Debug, Default)]
pub struct DataExplorerImpl {}

#[glib::object_subclass]
impl ObjectSubclass for DataExplorerImpl {
    const NAME: &'static str = "DataExplorerImpl";
    type Type = super::DataExplorer;
    type ParentType = gtk::Box;
}

impl ObjectImpl for DataExplorerImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let header = self.build_header();
        let ab = self.build_action_bar();
        let tv = self.build_tree();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.append(&header);
        container.append(&ab);
        container.append(&tv);

        let obj = self.obj();
        obj.set_vexpand(true);
        obj.set_hexpand(true);
        obj.append(&container);
    }
}

impl WidgetImpl for DataExplorerImpl {}

impl BoxImpl for DataExplorerImpl {}

impl DataExplorerImpl {
    pub fn build_header(&self) -> gtk::Box {
        let icon = gtk::Image::builder()
            .icon_name("data_source")
            .halign(gtk::Align::Start)
            .margin_start(12)
            .margin_top(4)
            .build();
        let label = gtk::Label::builder().label("Data Sources").build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        container.append(&icon);
        container.append(&label);

        return container;
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
        // let store = gio::ListStore::new::<glib::BoxedAnyObject>();

        // let model = gtk::TreeListModel::new(store, false, false, move |obj| {
        //     let boxed = obj
        //         .downcast_ref::<glib::BoxedAnyObject>()
        //         .expect("//todo BoxedAnyObject");
        //     return Self::create_child_model(&boxed);
        // });

        // let selection = gtk::SingleSelection::builder().model(&model).build();

        let tv = gtk::ColumnView::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        // .set_model(Some(&selection));
        // self.tv.append_column(&self.build_name_column());
        // self.tv.append_column(&self.build_menu_column());

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&tv)
            .build();

        return sw;
    }
}
