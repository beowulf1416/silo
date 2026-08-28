use gtk::gsk::PorterDuff::Source;
use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::{Ref, RefCell};

use sourceview5::prelude::*;

// use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;
use silo_plugin::node::Node;

#[derive(Debug, Default)]
pub struct QueryEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub(super) sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,

    // pub(super) sources: RefCell<Option<gio::ListStore>>,
    pub(super) cbo_sources: gtk::DropDown,
}

impl QueryEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }

    pub fn set_model(&self, sources: gio::ListStore) {
        // self.sources.replace(Some(sources.clone()));

        self.cbo_sources.set_model(Some(&sources.clone()))
    }

    fn build_sources_drop_down(&self) {
        // let sources_borrow = self.sources.borrow();
        // let model = sources_borrow.as_ref().clone().unwrap();

        // let cbo = gtk::DropDown::builder()
        //     .enable_search(false)
        //     .show_arrow(true)
        //     // .model(&model.clone().upcast::<gio::ListModel>())
        //     // .factory(&factory)
        //     .build();

        self.cbo_sources.set_enable_search(false);
        self.cbo_sources.set_show_arrow(true);
        self.cbo_sources.set_css_classes(&vec!["flat"]);

        // let sources_borrow = self.sources.borrow();
        // let ref_dsn = sources_borrow.as_ref();
        // if let Some(dsn) = ref_dsn {
        //     let model = dsn.clone();
        //     cbo.set_model(Some(&model));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, item| {
            let label = gtk::Label::builder().build();

            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("//todo gtk::ListItem");
            litem.set_child(Some(&label));
        });

        factory.connect_bind(|_, item| {
            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("//todo gtk::ListItem");

            let label = litem
                .child()
                .and_downcast::<gtk::Label>()
                .expect("//todo gtk::Label");

            let boxed = litem
                .item()
                .and_downcast::<glib::BoxedAnyObject>()
                .expect("//todo glib::BoxedAnyObject");

            let node_ref: Ref<Box<dyn Node>> = boxed.borrow::<Box<dyn Node>>();
            let node: &dyn Node = node_ref.as_ref();

            label.set_label(node.name());
        });

        self.cbo_sources.set_factory(Some(&factory));
        // }

        // return cbo;
    }

    fn build_action_bar(&self) -> gtk::ActionBar {
        let btn_save = gtk::Button::builder()
            // .label("Save")
            .icon_name("save")
            .tooltip_text("Save")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_save.connect_clicked(|_button| {
            debug!("button save clicked");
        });

        let separator = gtk::Separator::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let btn_execute = gtk::Button::builder()
            // .label("Execute")
            .icon_name("system-play-start")
            .tooltip_text("Execute")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_execute.connect_clicked(|_button| {
            debug!("button execute clicked");
        });

        self.build_sources_drop_down();

        let action_bar = gtk::ActionBar::builder()
            .hexpand(true)
            .css_classes(vec!["action-bar"])
            .build();
        action_bar.pack_start(&btn_save);
        action_bar.pack_start(&separator);
        action_bar.pack_start(&btn_execute);
        action_bar.pack_end(&self.cbo_sources);

        return action_bar;
    }

    fn build_editor(&self) -> gtk::ScrolledWindow {
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        if let Some(ref language) = sourceview5::LanguageManager::new().language("sql") {
            buffer.set_language(Some(language));
        }

        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
            buffer.set_style_scheme(Some(scheme));
        }

        let view = sourceview5::View::with_buffer(&buffer);
        view.set_monospace(true);
        view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        view.set_show_line_numbers(true);
        view.set_highlight_current_line(true);
        // view.set_highlight_matching_brackets(true);
        view.set_tab_width(4);
        view.set_hexpand(true);
        view.set_vexpand(true);

        let sv = gtk::ScrolledWindow::builder()
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&view)
            .build();

        return sv;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for QueryEditor {
    const NAME: &'static str = "QueryEditor";
    type Type = super::QueryEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for QueryEditor {
    fn constructed(&self) {
        self.parent_constructed();

        let action_bar = self.build_action_bar();
        let editor = self.build_editor();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.append(&action_bar);
        container.append(&editor);

        let obj = self.obj();
        obj.append(&container);
    }
}

impl WidgetImpl for QueryEditor {}

impl BoxImpl for QueryEditor {}
