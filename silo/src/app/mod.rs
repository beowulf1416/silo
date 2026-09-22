mod imp;

use adw::prelude::*;
use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};
use tracing::debug;

use crate::APP_ID;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::AppImpl>)
    @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
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
            .build();

        return app;
    }
}
