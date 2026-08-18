use async_channel::Sender;
use tracing::{debug, error};

use std::cell::Ref;

use gtk::{
    gio,
    gio::prelude::*,
    glib::{self, BoxedAnyObject, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::APP_TITLE;

#[derive(Debug, Clone)]
pub enum MainWindowInputMessage {
    CloseRequested,
}

#[derive(Debug, Default)]
pub struct MainWindow {}

impl MainWindow {
    fn add_actions(&self) {
        // let obj = self.obj().clone();

        // let preferences_action = crate::ui::actions::preferences::preferences_action(&obj);
        // self.obj().add_action(&preferences_action);

        // let about_action = crate::ui::actions::about::about_action(&obj);
        // self.obj().add_action(&about_action);

        // let file_open_action = crate::ui::actions::file_open::file_open_action(&obj);
        // self.obj().add_action(&file_open_action);

        // let open_workspace_action = crate::ui::actions::open_workspace::open_workspace_action(&obj);
        // self.obj().add_action(&open_workspace_action);
    }

    fn setup_action_handlers(&self) {
        // self.open_workspace_button.connect_clicked(move |btn| {
        //     debug!("Open Workspace button clicked");
        //     btn.activate_action("win.workspace.open", None);
        // });

        // self.connect_close_
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

        let (sender, receiver) = async_channel::unbounded::<MainWindowInputMessage>();

        obj.set_title(Some(APP_TITLE));
        obj.set_default_size(800, 600);

        self.add_actions();
        self.setup_action_handlers();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}
