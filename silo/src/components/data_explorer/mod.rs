mod imp;
pub mod node;

use gtk::{glib, prelude::*, subclass::prelude::*};
use std::sync::Arc;
use tracing::{debug, error, info};

use silo_api::{
    Silo,
    data_sources::DataSource,
    node::{Node, NodeType},
    plugin::Plugin,
};

use crate::app::App;
use crate::components::app_window::AppWindow;
// use node::{Node, NodeType};

glib::wrapper! {
    pub struct DataExplorer(ObjectSubclass<imp::DataExplorerImpl>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl DataExplorer {
    pub fn new(window: &AppWindow) -> Self {
        let obj: Self = glib::Object::builder().build();

        let imp = obj.imp();
        imp.window.replace(Some(window.clone()));

        //         debug!("app {:?}", window.application());
        //
        //         if let Some(app) = window.application().and_downcast::<App>() {
        //             let silo: Arc<dyn Silo> = app.silo();
        //             let sources = silo.sources();
        //
        //             for s in sources {
        //                 imp.store.append(&glib::BoxedAnyObject::new(Node::new(
        //                     &s.name,
        //                     NodeType::DataSource,
        //                     &s.plugin,
        //                     &s.data_source_name,
        //                 )));
        //             }
        //         } else {
        //             error!("unable to obtain App object");
        //         }

        return obj;
    }

    pub fn set_silo(&self, silo: Arc<dyn Silo>) {
        info!("DataExplorer::set_silo");

        let imp = self.imp();
        imp.set_silo(&silo);
    }

    pub fn register_source(&self, source: &DataSource, plugin: Option<Arc<dyn Plugin>>) {
        info!("DataExplorer::register_source()");

        let imp = self.imp();
        imp.register_source(source, plugin);
    }
}
