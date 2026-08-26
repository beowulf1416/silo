use gtk::{gio, glib, prelude::*, subclass::prelude::*};

pub trait Node: std::fmt::Debug {
    fn name(&self) -> &str;
    // fn children(&self) -> gio::ListStore;
    fn clone_box(&self) -> Box<dyn Node>;
    fn children(&self) -> Option<gio::ListStore>;

    fn context_menu(&self) -> Option<gio::Menu>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
