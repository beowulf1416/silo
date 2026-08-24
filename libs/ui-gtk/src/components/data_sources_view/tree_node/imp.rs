use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use std::cell::RefCell;
use std::sync::Arc;

use crate::components::data_sources_view::tree_node::Node;

#[derive(Debug, Default)]
pub struct TreeNode {
    // pub name: String,
    // pub children: RefCell<Option<Vec<Arc<TreeNode>>>>,
    pub node: RefCell<Option<Node>>,
}

impl TreeNode {
    pub fn node(&self) -> Node {
        return self.node.borrow().as_ref().expect("Node").clone();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TreeNode {
    const NAME: &'static str = "TreeNode";
    type Type = super::TreeNode;
}

impl ObjectImpl for TreeNode {}
