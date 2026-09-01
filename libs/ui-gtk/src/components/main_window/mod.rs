mod imp;

use async_channel::Receiver;
use gio::prelude::ActionGroupExt;
use gtk::glib::bitflags::serde;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use tracing::{debug, error, warn};

use serde_json::Value;
use std::cell::Ref;
use std::sync::Arc;

use crate::App;
use crate::components::editor_view::EditorView;

use silo_plugin::node::{DataSourceNode, Node};
use silo_plugin::{ApplicationMessage, StatusMessage};

use crate::components;

// type PluginFactory = fn() -> Box<dyn Plugin>;
type PluginName = String;
type WorkspacePath = String;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
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
        window.restore_state();

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

    pub fn editor_view(&self) -> EditorView {
        return self.imp().ev.clone();
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
            ApplicationMessage::Close => {
                debug!("process_message: close");
                if let Some(app) = self.application() {
                    app.quit();
                }
            }
            ApplicationMessage::CloseRequested => {
                debug!("process_message: close requested");
                // if let Some(app) = self.application() {
                //     app.quit();
                // }
                self.imp().close_requested();
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

                if let Err(e) = self.workspace_save() {
                    error!("workspace save failed: {}", e);

                    let imp = self.imp();
                    imp.notify(StatusMessage::Error(format!("{}", e)));
                }
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
                let imp = self.imp();

                let editor =
                    components::query_editor::QueryEditor::with_model(&self.data_sources());

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
        let imp = self.imp();
        imp.app
            .borrow()
            .clone()
            .expect("expecting App")
            .set_workspace_path(path.clone());
    }

    fn workspace_save(&self) -> anyhow::Result<()> {
        debug!("workspace_save");

        // check if workspace_path is set
        let imp = self.imp();
        let mut workspace_path = imp
            .app
            .borrow()
            .clone()
            .expect("expecting App")
            .workspace_path();
        debug!("workspace_path 1: {:?}", workspace_path);

        if workspace_path.is_none() {
            // let user choose a workspace path
            if let Some(action) = self.lookup_action("workspace-open") {
                action.activate(None);
            }

            workspace_path = imp
                .app
                .borrow()
                .clone()
                .expect("expecting App")
                .workspace_path();

            debug!("workspace_path 2: {:?}", workspace_path);
        };

        let sources = self.data_sources();

        let dsns: std::collections::HashMap<String, serde_json::Value> = sources
            .iter::<glib::BoxedAnyObject>()
            .map(|item| {
                let result = {
                    let node_ref: glib::BoxedAnyObject = item.expect("//todo glib::BoxedAnyObject");
                    node_ref
                };
                let node_ref: Ref<Arc<dyn Node>> = result.borrow();
                let node: &dyn Node = node_ref.as_ref();
                let dsn = node.into_DataSourceNode();
                (node.name().to_string(), dsn)
            })
            .filter(|item| item.1.is_some())
            .map(|item| (item.0.clone(), item.1.as_ref().unwrap().get_configuration()))
            .collect();

        if let Some(path) = workspace_path {
            let f = format!("{}/data_sources.json", path);
            let mut file = std::fs::File::create(std::path::Path::new(&f))?;
            serde_json::to_writer(&mut file, &dsns)?;

            imp.notify(StatusMessage::Info("Workspace saved".to_string()));
        } else {
            return Err(anyhow::anyhow!("Workspace path not set"));
        }

        return Ok(());
    }

    fn restore_state(&self) {
        debug!("restoring state...");

        let settings = gio::Settings::new(crate::APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        let _ = self.set_default_size(width, height);
        let _ = self.set_maximized(is_maximized);
    }
}
