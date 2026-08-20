mod imp;

use tracing::debug;

use std::rc::Rc;

use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};

use crate::{APP_ID, plugins::PluginRegistry};
// use silo_base::Silo;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    // pub fn set_silo(&self, silo: Silo) {
    //     self.imp().silo.set(Rc::new(silo)).ok();
    // }

    pub fn new() -> Self {
        debug!("mod::App::new()");

        gtk::init().unwrap();

        let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/org.devphilplus.silo.gresource"
        )));
        let resource = gio::Resource::from_data(&resource_bytes).unwrap();
        gio::resources_register(&resource);

        let app: App = glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            // .property("workspace_path")
            .build();

        app.initialize_registry();

        return app;
    }

    pub fn registry(&self) -> PluginRegistry {
        let imp = self.imp();
        return imp
            .registry
            .borrow()
            .as_ref()
            .expect("expecting PluginRegistry")
            .clone();
    }

    fn initialize_registry(&self) {
        debug!("init registry");

        let mut registry = PluginRegistry::new();
        registry.register("postgres", crate::plugins::postgres::factory);

        debug!("registry {:?}", registry);

        let imp = self.imp();
        imp.registry.replace(Some(registry));
    }
}

impl Default for App {
    fn default() -> Self {
        return Self::new();
    }
}
