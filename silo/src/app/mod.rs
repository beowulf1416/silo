mod imp;

use adw::prelude::*;
use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::info;

use crate::APP_ID;

use silo_api::Silo;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::AppImpl>)
    @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl App {
    pub fn new(silo: Arc<dyn Silo>) -> Self {
        info!("App::new");

        gtk::init().unwrap();

        let app: App = glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            .build();

        let imp = app.imp();
        imp.set_silo(silo);

        return app;
    }

    pub fn silo(&self) -> Arc<dyn Silo> {
        let imp = self.imp();
        return imp.silo.clone().into_inner().unwrap();
    }
}
