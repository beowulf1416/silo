mod imp;

use super::*;
// use std::cell::RefCell;
// use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

use crate::components::data_sources_view::node::Node;

glib::wrapper! {
    pub struct GNode(ObjectSubclass<imp::GNode>);
}

impl GNode {
    pub fn new(node: Arc<Node>) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().node.replace(Some(node));
        return obj;
    }

    pub fn node(&self) -> Arc<Node> {
        let imp = self.imp();
        return imp.node.borrow().as_ref().expect("Node expected").clone();
    }

    pub fn display_name(&self) -> String {
        return self.node().name().to_string();
    }

    pub fn children(&self) -> Vec<Arc<Node>> {
        return self.node().children().to_vec();
    }
}
