use adw::{prelude::*, subclass::prelude::*};
use gtk::prelude::GtkWindowExt;
use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use tracing::debug;

use crate::components::{data_explorer, editor_pane, status_bar};

enum MainWindowMessages {
    Close,
    CloseRequested,
}

#[derive(Debug, Default)]
pub struct MainWindowImpl {
    sender: RefCell<Option<async_channel::Sender<MainWindowMessages>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindowImpl {
    const NAME: &'static str = "MainWindowImpl";
    type Type = super::MainWindow;
    type ParentType = adw::ApplicationWindow;
}

impl ObjectImpl for MainWindowImpl {
    fn constructed(&self) {
        self.parent_constructed();

        self.init_channels();

        let header_bar = adw::HeaderBar::builder()
            .hexpand(true)
            // .title_widget(&self.header_bar)
            .build();

        let de = data_explorer::DataExplorer::new();
        let ep = editor_pane::EditorPane::new();
        let sb = status_bar::StatusBar::new();

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .position(200)
            .shrink_start_child(true)
            .resize_start_child(true)
            .wide_handle(true)
            .build();

        paned.set_start_child(Some(&de));
        paned.set_end_child(Some(&ep));

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();
        container.append(&header_bar);
        container.append(&paned);
        container.append(&sb);

        let obj = self.obj();
        obj.set_content(Some(&container));
    }
}

impl WindowImpl for MainWindowImpl {}

// impl GtkWindowExt for MainWindowImpl {}

impl WidgetImpl for MainWindowImpl {}

impl ApplicationWindowImpl for MainWindowImpl {}

impl AdwApplicationWindowImpl for MainWindowImpl {}

impl MainWindowImpl {
    pub fn init_channels(&self) {
        let (sender, receiver) = async_channel::unbounded::<MainWindowMessages>();
        self.sender.borrow_mut().replace(sender);

        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(msg) = receiver.recv().await {
                    window.process_message(msg);
                }
            }
        ));
    }

    fn process_message(&self, msg: MainWindowMessages) {
        match msg {
            MainWindowMessages::Close => {
                debug!("MainWindowMessages::Close");
            }
            MainWindowMessages::CloseRequested => {
                debug!("MainWindowMessages::CloseRequested");
            }
        }
    }
}
