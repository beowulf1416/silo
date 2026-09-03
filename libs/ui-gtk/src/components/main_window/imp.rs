use gtk::gio::SettingsBackend;
use tracing::{debug, error};

// use async_channel::Sender;
use std::cell::{OnceCell, RefCell};
// use std::collections::HashMap;
// use std::rc::Rc;
use std::borrow::Borrow;
use std::sync::Arc;

use adw::{prelude::*, subclass::prelude::*};
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

    pub(super) app: RefCell<Option<App>>,

    pub(super) data_sources: RefCell<Option<gio::ListStore>>,

    pub(super) workspace_path: RefCell<Option<String>>,
    pub(super) workspace_path_is_dirty: RefCell<bool>,
}

impl MainWindow {
    pub fn sender(&self) -> async_channel::Sender<ApplicationMessage> {
        return self
            .sender
            .borrow()
            .as_ref()
            .expect("sender is not set")
            .clone();
    }

    pub fn sender_status(&self) -> async_channel::Sender<StatusMessage> {
        return self
            .sender_status
            .borrow()
            .as_ref()
            .expect("sender_status is not set")
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

    pub fn set_workspace_path(&self, path: &String) -> anyhow::Result<()> {
        self.workspace_path.replace(Some(path.clone()));
        self.workspace_path_is_dirty.replace(true);

        // load data_sources.json in workspace_path
        match std::fs::File::open(format!("{}/data_sources.json", path.clone())) {
            Err(e) => {
                error!("Failed to open data_sources.json: {}", e);
                return Err(anyhow::anyhow!("Failed to open data_sources.json: {}", e));
            }
            Ok(mut file) => {
                let reader = std::io::BufReader::new(file);
                let config: serde_json::Value = serde_json::from_reader(reader)?;

                return Ok(());
            }
        }
    }

    pub fn workspace_path(&self) -> Option<String> {
        return self.workspace_path.borrow().clone();
    }

    pub fn is_workspace_path_dirty(&self) -> bool {
        return self.workspace_path_is_dirty.borrow().clone();
    }

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

    fn check_unsaved(&self) -> bool {
        debug!("check_unsaved");
        return true;
    }

    pub fn close_requested(&self) {
        let obj = self.obj();

        // check if there are unsaved changes
        // if so, show a confirmation dialog
        if self.check_unsaved() {
            debug!("alert dialog");

            let dialog = adw::AlertDialog::builder()
                .heading("Unsaved changes")
                .body("You have unsaved changes. Do you want to save them before closing?")
                .build();

            dialog.add_response("yes", "Yes");
            dialog.add_response("no", "No");
            dialog.set_default_response(Some("yes"));
            dialog.set_close_response("no");

            glib::MainContext::default().spawn_local(glib::clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    debug!("showing alert dialog");

                    let obj = this.obj();
                    let window = obj.clone().upcast::<gtk::Window>();

                    match dialog.choose_future(Some(&window)).await.as_str() {
                        "no" => {
                            debug!("chose no");
                            // return;
                        }
                        "yes" => {
                            debug!("chose yes");
                            this.save_settings();
                        }
                        _ => {
                            debug!("unknown response");
                        }
                    }

                    let _ = obj.send(ApplicationMessage::Close);
                }
            ));
        }
    }

    fn save_settings(&self) {
        debug!("save_settings");
        let window = self.obj();
        // let settings = gio::Settings::new(crate::APP_ID);

        // let schema_dir = std::path::Path::new("libs/ui-gtk");
        // if let Ok(source) = gio::SettingsSchemaSource::from_directory(schema_dir, None, false) {
        //     if let Some(schema) = source.lookup(crate::APP_ID, false) {
        //         let settings =
        //             gio::Settings::new_full(&schema, Option::<&SettingsBackend>::None, None);

        //         let (width, height) = window.default_size();
        //         let _ = settings.set_int("window-width", width);
        //         let _ = settings.set_int("window-height", height);

        //         let _ = settings.set_boolean("is-maximized", window.is_maximized());

        //         let workspace_path = self.workspace_path().unwrap_or(String::from(""));
        //         let _ = settings.set_string("workspace-path", workspace_path.as_str());
        //     }
        // } else {
        //     error!("Failed to load settings schema");
        // }

        if let Some(dirs) = directories::ProjectDirs::from("com", "devphilplus", "silo") {
            let config_dir = dirs.config_dir();
            let _ = std::fs::create_dir_all(&config_dir);
            let config_file = config_dir.join("config.json");

            // let (width, height) = window.default_size();
            let width = window.width();
            let height = window.height();

            let config = serde_json::json!({
                "window": {
                    "width": width,
                    "height": height,
                    "maximised": window.is_maximized()
                },
                "workspace_path": self.workspace_path().unwrap_or(String::from(""))
            });

            let _ = std::fs::write(&config_file, serde_json::to_string(&config).unwrap());
        }
    }

    pub fn restore_settings(&self) {
        debug!("restoring settings...");

        if let Some(dirs) = directories::ProjectDirs::from("com", "devphilplus", "silo") {
            let config_dir = dirs.config_dir();
            debug!("config_dir: {:?}", config_dir);

            let config_file = config_dir.join("config.json");

            if let Ok(data) = std::fs::read(&config_file) {
                if let Ok(config) = serde_json::from_slice::<serde_json::Value>(&data) {
                    if let Some(window) = config.get("window") {
                        if let Some(width) = window.get("width") {
                            if let Some(width) = width.as_u64() {
                                debug!("width: {}", width);
                                // self.obj().set_default_size(width as i32, -1);
                                self.obj().set_width_request(width as i32);
                            }
                        }

                        if let Some(height) = window.get("height") {
                            if let Some(height) = height.as_u64() {
                                debug!("height: {}", height);
                                // self.obj().set_default_size(-1, height as i32);
                                self.obj().set_height_request(height as i32);
                            }
                        }

                        if let Some(maximised) = window.get("maximised") {
                            if let Some(maximised) = maximised.as_bool() {
                                debug!("maximised: {}", maximised);
                                self.obj().set_maximized(maximised);
                            }
                        }
                    }

                    if let Some(workspace_path) = config.get("workspace_path") {
                        if let Some(workspace_path) = workspace_path.as_str() {
                            self.workspace_path
                                .replace(Some(workspace_path.to_string()));
                        }
                    }
                }
            }
        }
    }

    fn build_main_menu(&self) -> gio::Menu {
        let menu = gio::Menu::new();
        menu.append(Some("_File"), None);

        let section = gio::Menu::new();

        let item = gio::MenuItem::new(Some("Open ..."), Some("win.workspace-open"));
        section.append_item(&item);

        let item = gio::MenuItem::new(Some("Save"), Some("win.workspace-save"));
        section.append_item(&item);

        menu.append_section(Some("Workspace"), &section);

        let section = gio::Menu::new();
        let item = gio::MenuItem::new(Some("_Quit"), Some("win.quit"));
        section.append_item(&item);
        menu.append_section(None, &section);

        return menu;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    // type ParentType = gtk::ApplicationWindow;
    type ParentType = adw::ApplicationWindow;

    // fn class_init(klass: &mut Self::Class) {
    //     // klass.bind_template();
    //     // klass.bind_template_instance_callbacks();
    // }

    // fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
    //     // obj.init_template();
    // }
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

        // obj.set_default_size(800, 600);
        // obj.set_titlebar(Some(&self.header_bar));

        let header_bar = adw::HeaderBar::builder()
            .title_widget(&self.header_bar)
            .build();

        let menu = self.build_main_menu();
        let pop_menu = gtk::PopoverMenu::from_model(Some(&menu));

        let btn_menu = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Main Menu")
            .popover(&pop_menu)
            .build();
        header_bar.pack_start(&btn_menu);

        let toolbar = adw::ToolbarView::builder().content(&header_bar).build();

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        content_box.append(&toolbar);
        // obj.set_child(Some(&content_box));
        obj.set_content(Some(&content_box));

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

        return glib::Propagation::Stop;
    }
}

impl ApplicationWindowImpl for MainWindow {}

impl AdwApplicationWindowImpl for MainWindow {}
