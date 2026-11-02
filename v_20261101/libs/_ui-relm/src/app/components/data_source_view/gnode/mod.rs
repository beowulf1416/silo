pub mod imp;

use super::*;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

use super::*;

glib::wrapper! {
    pub struct GNode(ObjectSubclass<imp::GNode>);
}

impl GNode {
    pub fn new(node: Rc<dyn node::Node>) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().node.replace(Some(node));
        return obj;
    }

    pub fn node(&self) -> Rc<dyn node::Node> {
        return self.imp().node.borrow().clone().expect("expecting node");
    }
}
