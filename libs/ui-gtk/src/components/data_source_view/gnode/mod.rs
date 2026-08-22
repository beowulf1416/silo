mod imp;

use super::*;
// use std::cell::RefCell;
// use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

use super::*;
use node::Node;

glib::wrapper! {
    pub struct GNode(ObjectSubclass<imp::GNode>);
}

impl GNode {
    pub fn new(node: Arc<dyn Node>) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().node.replace(Some(node));
        return obj;
    }

    pub fn node(&self) -> Arc<dyn Node> {
        let imp = self.imp();
        return imp.node.borrow().as_ref().expect("Node expected").clone();
        // return self.imp().node.borrow().expect("Node expected");
        // .expect("expecting node");
    }

    pub fn display_name(&self) -> String {
        return self.node().display_name().to_string();
    }

    pub fn children(&self) -> Vec<Arc<dyn Node>> {
        return self.node().children().clone();
    }
}
