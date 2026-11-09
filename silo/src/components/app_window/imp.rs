use adw::{prelude::*, subclass::prelude::*};
use async_channel::Sender;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::Arc;
use tracing::{error, info};

use crate::actions::setup_actions;
use crate::components::data_explorer::DataExplorer;
use crate::components::*;

use silo_api::{AppWindowMessages, Silo, window::MainWindow};

// use super::AppWindowMessages;
// use crate::app::App;

#[derive(Debug, Default)]
pub struct AppWindowImpl {
    pub(super) sender: RefCell<Option<Sender<AppWindowMessages>>>,
    pub(super) silo: RefCell<Option<Arc<dyn Silo>>>,
    explorer: RefCell<Option<DataExplorer>>,
}

impl AppWindowImpl {
    pub fn set_silo(&self, silo: &Arc<dyn Silo>) {
        info!("AppWindowImpl::set_silo");

        self.silo.replace(Some(Arc::clone(silo)));

        // if let Some(explorer) = self.explorer.borrow().as_ref() {
        //     explorer.set_silo(Arc::clone(silo));
        // } else {
        //     error!("DataExplorer not set");
        // }

        if let Some(explorer) = self.explorer.borrow().as_ref() {
            for s in silo.sources() {
                let plugin = silo.plugin_manager().get_plugin(&s.plugin);

                explorer.register_source(&s, plugin);
            }
        } else {
            error!("AppWindowImpl::set_silo() unable to borrow DataExplorer");
        }
    }

    fn setup_channels(&self) {
        let (sender, receiver) = async_channel::unbounded::<AppWindowMessages>();
        self.sender.replace(Some(sender.clone()));

        // if let Some(mut silo) = self.silo.borrow_mut().as_ref() {
        //     silo.set_sender(sender.clone());
        // }

        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(msg) = receiver.recv().await {
                    window.process_message(&msg);
                }
            }
        ));
    }

    fn process_message(&self, msg: &AppWindowMessages) {
        info!("AppWindowImpl::process_message {:?}", msg);
        match msg {
            AppWindowMessages::Close => {
                if let Some(app) = self.obj().application() {
                    app.quit();
                }
            }
            AppWindowMessages::CloseRequested => {
                info!("Close Requested");
            }
            AppWindowMessages::ShowPasswordDialog => {
                info!("AppWindowImpl::process_message() AppWindowMessages::ShowPasswordDialog");

                self.show_password_dialog();
            }
        }
    }

    fn build_header_bar_menu(&self) -> gio::Menu {
        let menu = gio::Menu::new();

        menu.append(Some("_File"), None);

        let item = gio::MenuItem::new(Some("_Quit"), Some("win.quit"));
        let section = gio::Menu::new();
        section.append_item(&item);
        menu.append_section(None, &section);

        return menu;
    }

    fn build_status_bar(&self) -> gtk::Box {
        let container = gtk::Box::builder()
            .margin_start(4)
            .margin_end(4)
            .margin_top(4)
            .margin_bottom(4)
            .height_request(14)
            .hexpand(true)
            .build();

        return container;
    }

    pub fn show_password_dialog(&self) {
        info!("AppWindowImpl::show_password_dialog()");

        // let imp = self.imp();
        //
        let pd = password_dialog::PasswordDialog::new();
        //         pd.set_parent(&imp);
        //         pd.set_transient_for(Some(&imp));
        pd.set_modal(true);
        pd.present();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AppWindowImpl {
    const NAME: &'static str = "AppWindowImpl";
    type Type = super::AppWindow;
    type ParentType = adw::ApplicationWindow;
}

impl ObjectImpl for AppWindowImpl {
    fn constructed(&self) {
        info!("AppWindowImpl::constructed");

        self.parent_constructed();

        let obj = self.obj();
        let window = obj.borrow();

        let menu_btn = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Main Menu")
            .menu_model(&self.build_header_bar_menu())
            .build();

        let header_bar = adw::HeaderBar::builder().hexpand(true).build();
        header_bar.pack_start(&menu_btn);

        let data_explorer = data_explorer::DataExplorer::new(&window);
        self.explorer.replace(Some(data_explorer));

        let editor_pane = editor_pane::EditorPane::new();

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .vexpand(true)
            .position(200)
            .shrink_start_child(true)
            .resize_start_child(true)
            .wide_handle(true)
            .build();

        // paned.set_start_child(Some(&data_explorer));
        paned.set_start_child(self.explorer.borrow().as_ref());
        paned.set_end_child(Some(&editor_pane));

        let status_bar = self.build_status_bar();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.append(&header_bar);
        container.append(&paned);
        container.append(&status_bar);

        obj.set_content(Some(&container));

        setup_actions(&obj);

        self.setup_channels();
    }
}

impl WindowImpl for AppWindowImpl {}

impl WidgetImpl for AppWindowImpl {}

impl ApplicationWindowImpl for AppWindowImpl {}

impl AdwApplicationWindowImpl for AppWindowImpl {}

impl MainWindow for AppWindowImpl {
    fn show_password_dialog(&self) {
        info!("AppWindow::show_password_dialog()");
    }
}
