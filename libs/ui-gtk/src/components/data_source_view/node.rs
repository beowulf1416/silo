// use std::rc::Rc;
use std::sync::Arc;

pub trait Node: std::fmt::Debug {
    fn display_name(&self) -> &String;
    fn children(&self) -> &Vec<Arc<dyn Node>>;
}

#[derive(Debug)]
pub struct SimpleNode {
    pub name: String,
    pub children: Vec<Arc<dyn Node>>,
}

impl Node for SimpleNode {
    fn display_name(&self) -> &String {
        return &self.name;
    }

    fn children(&self) -> &Vec<Arc<dyn Node>> {
        return &self.children;
    }
}
