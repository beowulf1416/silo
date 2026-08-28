use std::future::Future;
use std::pin::Pin;

// use std::any::Any;
use std::vec::Vec;

use async_trait::async_trait;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[async_trait]
pub trait Node: std::fmt::Debug {
    fn name(&self) -> &str;
    // fn children(&self) -> gio::ListStore;
    fn clone_box(&self) -> Box<dyn Node>;

    fn children(&self) -> Option<Vec<Box<dyn Node>>>;
    async fn children_async(&self) -> Result<Option<Vec<Box<dyn Node>>>, &'static str>;

    fn context_menu(&self) -> Option<gio::Menu>;

    // fn populate_child_store<'a>(
    //     &'a self,
    //     store: &'a gio::ListStore,
    // ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    // fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait DataSourceNode: std::fmt::Debug {
    fn query(&self, sql: &str) -> Result<(), &'static str>;
}
