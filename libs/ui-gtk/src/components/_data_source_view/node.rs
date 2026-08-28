// use std::rc::Rc;
use std::sync::Arc;

pub trait Node: std::fmt::Debug {
    fn display_name(&self) -> &String;
    fn children(&self) -> &[Arc<dyn Node>];
}

#[derive(Debug, Clone)]
pub struct SimpleNode {
    pub name: String,
    pub children: Vec<Arc<dyn Node>>,
}

impl Node for SimpleNode {
    fn display_name(&self) -> &String {
        return &self.name;
    }

    fn children(&self) -> &[Arc<dyn Node>] {
        return &self.children;
    }
}

#[derive(Debug, Clone)]
pub struct ObjectTypeNode {
    pub name: String,
    pub children: Vec<Arc<dyn Node>>,
}

impl Node for ObjectTypeNode {
    fn display_name(&self) -> &String {
        return &self.name;
    }

    fn children(&self) -> &[Arc<dyn Node>] {
        return &self.children;
    }
}

#[derive(Debug, Clone)]
pub struct DataSourceNode {
    pub name: String,
    children: Vec<Arc<dyn Node>>,
    pub config: serde_json::Value,
}

impl DataSourceNode {
    pub fn new(
        name: String,
        children: Vec<Arc<ObjectTypeNode>>,
        config: serde_json::Value,
    ) -> Self {
        return Self {
            name: name,
            children: children
                .into_iter()
                .map(|n| n.clone() as Arc<dyn Node>)
                .collect(),
            config: config,
        };
    }
}

impl Node for DataSourceNode {
    fn display_name(&self) -> &String {
        return &self.name;
    }

    fn children(&self) -> &[Arc<dyn Node>] {
        return &self.children;
    }
}
