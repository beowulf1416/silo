mod imp;

use async_channel::Receiver;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use tracing::{debug, error, warn};

use serde_json::Value;
use std::sync::Arc;

use crate::App;
// use crate::components::data_sources_view::node::Node;
// use crate::components::data_source_view::node::SimpleNode;
// use crate::components::data_sources_view::tree_node::data_source_node::DataSourceNode;
use crate::plugins::Plugin;
use silo_plugin::ApplicationMessage;

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

    pub fn start_receiver(&self, receiver: Receiver<ApplicationMessage>) {
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

                let imp = self.imp();
                imp.dsv.data_source_add(box_node);
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
