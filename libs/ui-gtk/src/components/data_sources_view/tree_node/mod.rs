pub mod data_source_node;
mod imp;
pub mod schema_node;
pub mod schema_object_node;

use std::cell::RefCell;
use std::sync::Arc;

use tracing::debug;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Clone)]
pub enum Node {
    DataSourceNode(data_source_node::DataSourceNode),
    SchemaNode(schema_node::SchemaNode),
    SchemaObjectNode(schema_object_node::SchemaObjectNode),
    TableNode,
}

glib::wrapper! {
    pub struct TreeNode(ObjectSubclass<imp::TreeNode>);
}

impl TreeNode {
    pub fn new(node: Node) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().node.replace(Some(node));
        return obj;
    }

    pub fn node(&self) -> Node {
        return self.imp().node();
    }

    pub fn name(&self) -> String {
        let node = self.imp().node();
        match node {
            Node::DataSourceNode(dsn) => dsn.name,
            Node::SchemaNode(sn) => sn.name,
            Node::SchemaObjectNode(son) => son.name,
            Node::TableNode => "//todo: TableNode".to_string(),
        }
    }
}
