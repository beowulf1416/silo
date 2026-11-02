use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::sync::Arc;
use tracing::{error, info};

use silo_api::Silo;

use crate::components::app_window::AppWindow;

#[derive(Debug, Default)]
pub struct AppImpl {
    pub(super) silo: RefCell<Option<Arc<dyn Silo>>>,

    pub(super) window: RefCell<Option<Arc<AppWindow>>>,
}

impl AppImpl {
    pub fn set_silo(&self, silo: Arc<dyn Silo>) {
        info!("AppImpl::set_silo");

        self.silo.replace(Some(Arc::clone(&silo)));

        // if let Some(window) = self.window.borrow().as_ref() {
        //     window.set_silo(Arc::clone(&silo));
        // } else {
        //     error!("AppWindow not set");
        // }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AppImpl {
    const NAME: &'static str = crate::APP_NAME;
    type Type = super::App;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppImpl {}

impl ApplicationImpl for AppImpl {
    fn activate(&self) {
        info!("AppImpl::activate()");

        self.parent_activate();

        let obj = self.obj();
        let app = obj.as_ref();

        let window = AppWindow::new(&app);
        if let Some(silo) = self.silo.borrow().as_ref() {
            window.set_silo(silo);
            silo.set_window(Arc::new(window.clone()));
        } else {
            error!("Silo not set");
        }
        window.present();

        self.window.replace(Some(Arc::new(window)));
    }
}

impl GtkApplicationImpl for AppImpl {}
