use tracing::{debug, error};

// use async_channel::Sender;
use std::cell::{OnceCell, RefCell};
// use std::collections::HashMap;
// use std::rc::Rc;
use std::sync::Arc;

use gtk::{
    gio,
    gio::prelude::*,
    glib::{self, BoxedAnyObject, clone},
    prelude::*,
    subclass::prelude::*,
};

// use super::MainWindowInputMessage;
use silo_plugin::node::Node;
use silo_plugin::{ApplicationMessage, StatusMessage};

use crate::{
    APP_TITLE,
    app::App,
    // components::data_source_view::DataSourceView,
    components::data_sources_view::DataSourcesView,
    plugins::PluginRegistry,
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
    pub sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,
    pub sender_status: RefCell<Option<async_channel::Sender<StatusMessage>>>,

    pub(super) header_bar: Header,
    pub(super) info: gtk::Label,
    pub(super) dsv: RefCell<Option<DataSourcesView>>,
    pub(super) ev: EditorView,

    // pub plugins: HashMap<String, Box<dyn Plugin>>,
    // pub registry: Rc<PluginRegistry>,
    pub(super) app: RefCell<Option<App>>,

    pub(super) data_sources: RefCell<Option<gio::ListStore>>,
}

impl MainWindow {
    // pub fn set_workspace_path(&self, new_path: String) {
    //     self.app
    //         .borrow()
    //         .expect("App struct expected")
    //         .set_workspace_path(new_path);
    // }

    pub fn sender(&self) -> async_channel::Sender<ApplicationMessage> {
        return self
            .sender
            .borrow()
            .as_ref()
            .expect("sender is not set")
            .clone();
    }

    pub fn notify(&self, message: StatusMessage) {
        let sender = self
            .sender_status
            .borrow()
            .as_ref()
            .expect("sender is not set")
            .clone();

        glib::MainContext::default().spawn_local(async move {
            let _ = sender.send(message).await;
        });
    }

    // pub fn send(&self, message: ApplicationMessage) {
    //     let sender = self
    //         .sender
    //         .borrow()
    //         .as_ref()
    //         .expect("sender is not set")
    //         .clone();

    //     glib::MainContext::default().spawn(async move {
    //         let _ = sender.send(message).await;
    //     });
    // }

    fn add_actions(&self) {
        let obj = self.obj().clone();
        crate::actions::setup_actions(&obj);
    }

    fn setup_action_handlers(&self) {}

    fn data_sources(&self) -> gio::ListStore {
        let mut sg = self.data_sources.borrow_mut();
        return sg
            .get_or_insert_with(|| gio::ListStore::new::<glib::BoxedAnyObject>())
            .clone();
    }

    fn build_dsv(&self) -> DataSourcesView {
        let dsv = DataSourcesView::with_model(&self.data_sources());
        dsv.set_sender(
            self.sender.borrow().clone().unwrap(),
            self.sender_status.borrow().clone().unwrap(),
        );
        return dsv;
    }

    fn build_status_bar(&self) -> gtk::Box {
        self.info.set_label(&"Ready");
        self.info.set_height_request(40);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(false)
            .tooltip_markup("Status Bar")
            .margin_start(10)
            .margin_end(10)
            .build();

        container.append(&self.info);

        return container;
    }
}

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
        // debug!("plugins {:?}", obj.application());

        let (sender, receiver) = async_channel::unbounded::<ApplicationMessage>();
        self.sender.replace(Some(sender));

        let (sender, receiver_status) = async_channel::unbounded::<StatusMessage>();
        self.sender_status.replace(Some(sender));

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

        let dsv = self.build_dsv();
        self.dsv.replace(Some(dsv.clone()));

        paned.set_start_child(Some(&dsv.clone()));
        paned.set_end_child(Some(&self.ev));

        let status_box = self.build_status_bar();
        content_box.append(&status_box);

        self.add_actions();
        self.setup_action_handlers();

        obj.start_receivers(receiver, receiver_status);
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {
    fn close_request(&self) -> glib::Propagation {
        debug!("MainWindow::close_request");

        let obj = self.obj();
        obj.send(ApplicationMessage::CloseRequested);

        return glib::Propagation::Proceed;
    }
}

impl ApplicationWindowImpl for MainWindow {}
