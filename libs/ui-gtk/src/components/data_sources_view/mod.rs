// pub mod gnode;
mod imp;
pub mod node;
// pub mod tree_node;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// use crate::components::data_sources_view::tree_node::{Node, data_source_node::DataSourceNode};
use crate::components::data_sources_view::node::Node;

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

    // pub fn data_source_add(&self, node: DataSourceNode) {
    //     let imp = self.imp();
    //     imp.data_source_add(node);
    // }
    pub fn data_source_add(&self, node: Box<dyn Node>) {
        let imp = self.imp();
        imp.data_source_add(node);
    }
}

impl Default for DataSourcesView {
    fn default() -> Self {
        return Self::new();
    }
}
