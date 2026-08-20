mod imp;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::{App, plugins::PluginRegistry};

glib::wrapper! {
    pub struct DataSourceView(ObjectSubclass<imp::DataSourceView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl DataSourceView {
    pub fn new(app: &App) -> Self {
        debug!("DataSourceView::new {:?}", app);

        let window: Self = glib::Object::builder().build();

        let registry = app.registry();
        window.set_registry(&registry);

        return window;
    }

    pub fn set_registry(&self, registry: &PluginRegistry) {
        debug!("set_registry {:?}", registry);

        let imp = self.imp();
        imp.registry.replace(Some(registry.clone()));
    }

    pub fn registry(&self) -> PluginRegistry {
        let imp = self.imp();
        return imp
            .registry
            .borrow()
            .as_ref()
            .expect("expecting App struct")
            .clone();
    }
}

impl Default for DataSourceView {
    fn default() -> Self {
        return Self::new(&crate::app::App::new());
    }
}
