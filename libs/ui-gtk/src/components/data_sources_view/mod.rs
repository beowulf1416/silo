mod imp;

use async_trait::async_trait;
use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use silo_plugin::node::Node;

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

    pub fn with_model(sources: &gio::ListStore) -> Self {
        let obj: Self = glib::Object::builder().build();
        let imp = obj.imp();
        imp.set_model(sources.clone());

        return obj;
    }

    // pub fn data_source_add(&self, node: DataSourceNode) {
    //     let imp = self.imp();
    //     imp.data_source_add(node);
    // }
    // pub fn data_source_add(&self, node: Box<dyn Node>) {
    //     let imp = self.imp();
    //     imp.data_source_add(node);
    // }

    // pub fn sources(&self) -> gio::ListStore {
    //     let imp = self.imp();
    //     return imp.sources();
    // }
}

impl Default for DataSourcesView {
    fn default() -> Self {
        return Self::new();
    }
}

#[derive(Debug, Clone)]
pub struct LoadingNode {}

#[async_trait]
impl Node for LoadingNode {
    fn name(&self) -> &str {
        return "Loading...";
    }

    fn children(&self) -> Option<Vec<Box<dyn Node>>> {
        return None;
    }

    async fn children_async(&self) -> Result<Option<Vec<Box<dyn Node>>>, &'static str> {
        return Err("//todo not implemented");
    }

    fn context_menu(&self) -> Option<gio::Menu> {
        return None;
    }

    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(self.clone());
    }

    // fn as_any(&self) -> &dyn std::any::Any {
    //     return self;
    // }
}
