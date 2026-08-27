use std::future::Future;
use std::pin::Pin;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

pub trait Node: std::fmt::Debug {
    fn name(&self) -> &str;
    // fn children(&self) -> gio::ListStore;
    fn clone_box(&self) -> Box<dyn Node>;
    fn children(&self) -> Option<Vec<Box<dyn Node>>>;

    fn context_menu(&self) -> Option<gio::Menu>;

    // fn populate_child_store<'a>(
    //     &'a self,
    //     store: &'a gio::ListStore,
    // ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
