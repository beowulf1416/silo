use super::*;

use std::cell::RefCell;
// use std::rc::Rc;
use std::sync::Arc;

use gtk::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

#[derive(Default)]
pub struct GNode {
    pub node: RefCell<Option<Arc<dyn node::Node>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GNode {
    const NAME: &'static str = "GNode";
    type Type = super::GNode;
}

impl ObjectImpl for GNode {}
