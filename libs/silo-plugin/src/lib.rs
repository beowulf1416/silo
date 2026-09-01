pub mod node;
pub mod plugin;

use tracing::debug;

use std::sync::Arc;

// use std::collections::HashMap;

pub type PluginName = String;
pub type DataSourceName = String;
pub type WorkspacePath = String;

pub enum ApplicationMessage {
    Close,
    CloseRequested,
    WorkspaceChanged(WorkspacePath),
    WorkspaceSaveRequested,
    NewQueryEditorRequested(DataSourceName),
    CloseEditorRequested(Option<u32>),
    NewDataSourceRequested(PluginName),
    DataSourceAdd(Arc<dyn node::Node>),
}

pub enum StatusMessage {
    Info(String),
    Error(String),
}

// pub trait Plugin: std::fmt::Debug {
//     fn register(&self) -> Result<(), &'static str>;
// }
