// pub mod gnode;
mod imp;
pub mod tree_node;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::components::data_sources_view::

glib::wrapper! {
    pub struct DataSourcesView(ObjectSubclass<imp::DataSourcesView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget
    ;
}

impl DataSourcesView {
    pub fn new() -> Self {
        debug!("DataSourcesView::new");

        let window: Self = glib::Object::builder().build();

        return window;
    }
}

impl Default for DataSourcesView {
    fn default() -> Self {
        return Self::new();
    }
}
