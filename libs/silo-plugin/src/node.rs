use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// use std::any::Any;
use std::vec::Vec;

use async_trait::async_trait;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

#[async_trait]
pub trait Node: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &str;
    // fn children(&self) -> gio::ListStore;
    // fn clone_box(&self) -> Arc<dyn Node>;

    fn children(&self) -> Option<Vec<Arc<dyn Node>>>;
    async fn children_async(&self) -> anyhow::Result<Option<Vec<Arc<dyn Node>>>>;

    fn context_menu(&self) -> Option<gio::Menu>;

    // fn populate_child_store<'a>(
    //     &'a self,
    //     store: &'a gio::ListStore,
    // ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    // fn as_any(&self) -> &dyn Any;

    // fn is_DataSourceNode(&self) -> bool;
    fn into_DataSourceNode(&self) -> Option<Arc<dyn DataSourceNode>>;
}

// impl Clone for Box<dyn Node> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

#[async_trait]
pub trait DataSourceNode: std::fmt::Debug + Send + Sync {
    async fn query(&self, sql: &str) -> anyhow::Result<()>;
}
