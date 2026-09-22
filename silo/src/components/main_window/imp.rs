use adw::{prelude::*, subclass::prelude::*};
use gtk::prelude::GtkWindowExt;
use gtk::{gio, gio::prelude::*, glib, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use tracing::{debug, error};

use super::MainWindowMessages;
use crate::components::{close_request_dialog, data_explorer, editor_pane, status_bar};

#[derive(Debug, Serialize, Deserialize)]
struct ApplicationSettings {
    workspace_path: Option<String>,
    window_maximized: Option<bool>,
    width: Option<i32>,
    height: Option<i32>,
}

#[derive(Debug, Default)]
pub struct MainWindowImpl {
    pub(super) sender: RefCell<Option<async_channel::Sender<MainWindowMessages>>>,
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

impl WindowImpl for MainWindowImpl {
    fn close_request(&self) -> glib::Propagation {
        debug!("MainWindowImpl::close_request");

        // save settings before closing
        self.save_settings();

        let obj = self.obj();
        obj.send(MainWindowMessages::CloseRequested);

        return glib::Propagation::Stop;
    }
}

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

    pub fn save_settings(&self) {
        debug!("MainWindowImpl::save_settings");

        let obj = self.obj();

        if let Some(dirs) = directories::ProjectDirs::from("com", "devphilplus", "silo") {
            let config_dir = dirs.config_dir();
            let _ = std::fs::create_dir_all(&config_dir);
            let config_file = config_dir.join("config.json");
            debug!("config_file: {:?}", config_file);

            let width = obj.width();
            let height = obj.height();

            let settings = ApplicationSettings {
                workspace_path: None,
                window_maximized: None,
                width: Some(width),
                height: Some(height),
            };

            let _ = std::fs::write(&config_file, serde_json::to_string(&settings).unwrap());
        }
    }

    pub fn restore_settings(&self) {
        debug!("MainWindowImpl::restore_settings");

        if let Some(dirs) = directories::ProjectDirs::from("com", "devphilplus", "silo") {
            let config_dir = dirs.config_dir();
            let config_file = config_dir.join("config.json");
            debug!("config_file: {:?}", config_file);

            if let Ok(contents) = std::fs::read_to_string(&config_file) {
                if let Ok(settings) = serde_json::from_str::<ApplicationSettings>(&contents) {
                    debug!("settings: {:?}", settings);

                    let obj = self.obj();
                    obj.set_maximized(settings.window_maximized.unwrap_or(false));
                    obj.set_default_size(
                        settings.width.unwrap_or(800),
                        settings.height.unwrap_or(600),
                    );
                }
            }
        }
    }

    fn process_message(&self, msg: MainWindowMessages) {
        match msg {
            MainWindowMessages::Close => {
                debug!("MainWindowMessages::Close");
                if let Some(app) = self.obj().application() {
                    app.quit();
                }
            }
            MainWindowMessages::CloseRequested => {
                debug!("MainWindowMessages::CloseRequested");
                self.show_close_request_dialog();
            }
        }
    }

    fn show_close_request_dialog(&self) {
        debug!("MainWindowImpl::show_close_request_dialog");

        let obj = self.obj();
        if let Some(app) = obj.app() {
            let window = obj.upcast_ref::<gtk::Window>();

            // let sender = self.sender.borrow().clone().unwrap();
            let dialog = close_request_dialog::CloseRequestDialog::new(&app, &obj);
            dialog.set_transient_for(Some(window));
            dialog.present();
        } else {
            error!("unable to show close request dialog: no application");
        }

        debug!("MainWindowImpl::show_close_request_dialog");
    }
}
