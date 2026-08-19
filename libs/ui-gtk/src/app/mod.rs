mod imp;

use tracing::debug;

use std::rc::Rc;

use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};

use crate::APP_ID;
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
        gtk::init().unwrap();

        let resource_bytes = glib::Bytes::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/org.devphilplus.silo.gresource"
        )));
        let resource = gio::Resource::from_data(&resource_bytes).unwrap();
        gio::resources_register(&resource);

        let app = glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            // .property("workspace_path")
            .build();

        return app;
    }
}

impl Default for App {
    fn default() -> Self {
        return Self::new();
    }
}
