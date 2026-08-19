use tracing::{debug, error};

use async_channel::Sender;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::{
    gio,
    gio::prelude::*,
    glib::{self, BoxedAnyObject, clone},
    prelude::*,
    subclass::prelude::*,
};

use super::MainWindowInputMessage;
use crate::{
    APP_TITLE,
    components::data_source_view::DataSourceView,
    plugins::{PluginRegistry, postgres::PostgresPlugin},
};
use crate::{
    components::{editor_view::EditorView, header::Header},
    plugins::Plugin,
};

// #[derive(Debug, Clone)]
// pub enum MainWindowInputMessage {
//     CloseRequested,
// }

#[derive(Debug, Default)]
pub struct MainWindow {
    pub sender: RefCell<Option<async_channel::Sender<MainWindowInputMessage>>>,
    pub header_bar: Header,
    // pub pane: gtk::Paned,
    pub dsv: DataSourceView,
    pub ev: EditorView,

    // pub plugins: HashMap<String, Box<dyn Plugin>>,
    pub registry: Rc<PluginRegistry>,
}

impl MainWindow {
    fn add_actions(&self) {
        let obj = self.obj().clone();

        // let action_group = gio::SimpleActionGroup::new();

        let quit_action = crate::actions::quit::quit_action(&obj);
        obj.add_action(&quit_action);

        let data_source_add_action = crate::actions::data_source_add::data_source_add_action(&obj);
        obj.add_action(&data_source_add_action);

        let data_source_remove_action =
            crate::actions::data_source_remove::data_source_remove_action(&obj);
        obj.add_action(&data_source_remove_action);
    }

    fn setup_action_handlers(&self) {
        // self.open_workspace_button.connect_clicked(move |btn| {
        //     debug!("Open Workspace button clicked");
        //     btn.activate_action("win.workspace.open", None);
        // });

        // self.connect_close_request
    }

    pub fn send(&self, msg: MainWindowInputMessage) {
        debug!("send (imp)");
        if let Some(sender) = self.sender.borrow().as_ref() {
            let _ = sender.send_blocking(msg);
        }
    }
}

// impl Default for MainWindow {
//     fn default() -> Self {
//         return Self {
//             sender: RefCell::default(),
//             header_bar: Header::default(),
//             dsv: DataSourceView::default(),
//             ev: EditorView::default(),
//             plugins: HashMap::new(),
//         };
//     }
// }

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // klass.bind_template();
        // klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        // obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // register plugins
        // let r = obj.application();
        debug!("plugins {:?}", obj.application());

        let (sender, receiver) = async_channel::unbounded::<MainWindowInputMessage>();
        *self.sender.borrow_mut() = Some(sender);

        obj.set_default_size(800, 600);
        obj.set_titlebar(Some(&self.header_bar));

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        obj.set_child(Some(&content_box));

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .position(200)
            .shrink_start_child(true)
            .resize_start_child(true)
            .build();
        content_box.append(&paned);

        paned.set_start_child(Some(&self.dsv));
        paned.set_end_child(Some(&self.ev));

        // status bar
        let status_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(false)
            .height_request(40)
            .tooltip_text("Status bar")
            .build();
        content_box.append(&status_box);

        self.add_actions();
        self.setup_action_handlers();

        obj.start_receiver(receiver);
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {
    fn close_request(&self) -> glib::Propagation {
        debug!("MainWindow::close_request");

        self.send(MainWindowInputMessage::CloseRequested);

        return glib::Propagation::Proceed;
    }
}

impl ApplicationWindowImpl for MainWindow {}
