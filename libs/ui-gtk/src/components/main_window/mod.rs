mod imp;

use async_channel::Receiver;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use tracing::{debug, error, warn};

use std::borrow::BorrowMut;

use serde_json::Value;
use std::sync::Arc;

use crate::App;

use silo_plugin::node::Node;
use silo_plugin::{ApplicationMessage, StatusMessage};

use crate::components;

// type PluginFactory = fn() -> Box<dyn Plugin>;
type PluginName = String;
type WorkspacePath = String;

// #[derive(Debug, Clone)]
// pub enum MainWindowInputMessage {
//     CloseRequested,
//     WorkspaceChanged(WorkspacePath),
//     NewDataSourceRequest(PluginName),
//     DataSourceAdd(Box<dyn Node>),
// }

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup
    ;
}

// #[gtk::template_callbacks]
impl MainWindow {
    pub fn new(application: &App) -> Self {
        debug!("MainWindow::new");

        let window: Self = glib::Object::builder()
            .property("application", application)
            .build();

        window.set_app(&application);

        return window;
    }

    pub fn set_app(&self, app: &App) {
        let imp = self.imp();
        imp.app.replace(Some(app.clone()));
    }

    fn app(&self) -> App {
        let imp = self.imp();
        return imp
            .app
            .borrow()
            .as_ref()
            .expect("expecting App struct")
            .clone();
    }

    fn data_sources(&self) -> gio::ListStore {
        let imp = self.imp();
        let mut sg = imp.data_sources.borrow_mut();

        return sg
            .get_or_insert_with(|| gio::ListStore::new::<glib::BoxedAnyObject>())
            .clone();
    }

    pub fn data_source_add(&self, dsn: Arc<dyn Node>) {
        let sources = self.data_sources();
        sources.append(&glib::BoxedAnyObject::new(dsn));
    }

    pub fn start_receivers(
        &self,
        receiver: Receiver<ApplicationMessage>,
        receiver_status: Receiver<StatusMessage>,
    ) {
        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(msg) = receiver.recv().await {
                    window.process_message(msg);
                }
            }
        ));

        glib::MainContext::default().spawn_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(msg) = receiver_status.recv().await {
                    window.process_notifications(msg);
                }
            }
        ));
    }

    fn process_notifications(&self, msg: StatusMessage) {
        match msg {
            StatusMessage::Error(message) => {
                debug!("show error: {}", message);
                self.imp().info.set_label(&message);
            }
            StatusMessage::Info(message) => {
                debug!("show info: {}", message);
                self.imp().info.set_label(&message);
            }
        }
    }

    fn process_message(&self, msg: ApplicationMessage) {
        match msg {
            ApplicationMessage::CloseRequested => {
                debug!("process_message: close requested");
                if let Some(app) = self.application() {
                    app.quit();
                }
            }
            ApplicationMessage::CloseEditorRequested(page) => {
                debug!("close editor requested: {:?}", page);
                self.imp().ev.remove_editor(page);
            }
            ApplicationMessage::WorkspaceChanged(workspace_path) => {
                debug!("workspace changed {}", workspace_path);
                self.set_workspace_path(&workspace_path);
            }
            ApplicationMessage::WorkspaceSaveRequested => {
                debug!("//todo workspace save requested");
            }
            ApplicationMessage::NewDataSourceRequested(plugin_name) => {
                debug!(
                    "process message: new data source requested: {}",
                    plugin_name
                );

                if let Some(plugin) = self.app().registry().create_plugin(&plugin_name) {
                    debug!("plugin {:?}", plugin);

                    if let Some(widget) =
                        plugin.build_data_source_editor_widget(self.imp().sender())
                    {
                        let imp = self.imp();
                        imp.ev.add_editor(
                            &plugin_name,
                            widget,
                            self.imp().sender.borrow().as_ref().unwrap(),
                        );
                    } else {
                        warn!("plugin {} does not have a data source editor", plugin_name);
                    }
                } else {
                    error!("unable to find plugin {}", plugin_name);
                }
            }
            ApplicationMessage::DataSourceAdd(box_node) => {
                debug!("DataSourceAdd {:?}", box_node);

                // let imp = self.imp();
                // // imp.dsv.data_source_add(box_node);
                // imp.data_source_add(box_node);
                self.data_source_add(box_node);
            }
            ApplicationMessage::NewQueryEditorRequested(plugin_name) => {
                // if let Some(plugin) = self.app().registry().create_plugin(&plugin_name) {
                //     debug!("plugin {:?}", plugin);

                //     if let Some(widget) = plugin.build_query_editor_widget(self.imp().sender()) {
                //         let imp = self.imp();
                //         imp.ev.add_editor(
                //             &plugin_name,
                //             widget,
                //             self.imp().sender.borrow().as_ref().unwrap(),
                //         );
                //     } else {
                //         warn!("plugin {} does not have a query editor", plugin_name);
                //     }
                // } else {
                //     error!("unable to find plugin {}", plugin_name);
                // }

                let imp = self.imp();

                let editor =
                    components::query_editor::QueryEditor::with_model(&self.data_sources());

                // let ds = self.data_sources();
                // debug!("data sources {:?}", ds);

                // editor.set_data_sources(self.data_sources());

                imp.ev.add_editor(
                    "test",
                    editor.upcast(),
                    self.imp().sender.borrow().as_ref().unwrap(),
                );
            }
        }
    }

    pub fn send(&self, msg: ApplicationMessage) {
        debug!("send (mod)");
        if let Some(sender) = self.imp().sender.borrow().as_ref() {
            let _ = sender.send_blocking(msg);
        }
    }

    fn set_workspace_path(&self, path: &String) {
        debug!("set_workspace_path {}", path);
    }
}
