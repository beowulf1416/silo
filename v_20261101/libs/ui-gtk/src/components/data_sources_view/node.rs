use gtk::{gio, glib, prelude::*, subclass::prelude::*};

pub trait Node: std::fmt::Debug {
    fn name(&self) -> &str;
    // fn children(&self) -> gio::ListStore;
    fn clone_box(&self) -> Box<dyn Node>;
    fn children(&self) -> Option<gio::ListStore>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
