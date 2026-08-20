mod imp;

use async_channel::Receiver;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use tracing::{debug, error};

use crate::App;
use crate::plugins::Plugin;

type PluginFactory = fn() -> Box<dyn Plugin>;
type PluginName = String;

#[derive(Debug, Clone)]
pub enum MainWindowInputMessage {
    CloseRequested,
    NewDataSourceRequest(PluginName),
}

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

    pub fn start_receiver(&self, receiver: Receiver<MainWindowInputMessage>) {
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

    fn process_message(&self, msg: MainWindowInputMessage) {
        match msg {
            MainWindowInputMessage::CloseRequested => {
                debug!("process_message: close requested");
                if let Some(app) = self.application() {
                    app.quit();
                }
            }
            MainWindowInputMessage::NewDataSourceRequest(plugin_name) => {
                debug!(
                    "process message: new data source requested: {}",
                    plugin_name
                );

                if let Some(plugin) = self.app().registry().create_plugin(&plugin_name) {
                    debug!("plugin {:?}", plugin);

                    let imp = self.imp();
                    imp.ev.add_editor(plugin);
                } else {
                    error!("unable to find plugin {}", plugin_name);
                }
            }
        }
    }

    pub fn send(&self, msg: MainWindowInputMessage) {
        debug!("send (mod)");
        if let Some(sender) = self.imp().sender.borrow().as_ref() {
            let _ = sender.send_blocking(msg);
        }
    }
}
