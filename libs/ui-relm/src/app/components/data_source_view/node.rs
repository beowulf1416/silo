// pub mod gnode;
// pub mod sql_db;

use std::fmt::Debug;
use std::rc::Rc;

pub trait Node: Debug {
    fn display_name(&self) -> &String;
    fn children(&self) -> Vec<Rc<dyn Node>>;
}

#[derive(Debug)]
pub struct SimpleNode {
    pub name: String,
    pub children: Vec<Rc<dyn Node>>,
}

impl Node for SimpleNode {
    fn display_name(&self) -> &String {
        return &self.name;
    }

    fn children(&self) -> Vec<Rc<dyn Node>> {
        return self.children.clone();
    }
}
