pub mod node;
pub mod plugin;

use tracing::debug;

use std::collections::HashMap;

pub type PluginName = String;
pub type WorkspacePath = String;

pub enum ApplicationMessage {
    CloseRequested,
    WorkspaceChanged(WorkspacePath),
    WorkspaceSaveRequested,
    CloseEditorRequested(Option<u32>),
    NewDataSourceRequested(PluginName),
    DataSourceAdd(Box<dyn node::Node>),
}

// pub trait Plugin: std::fmt::Debug {
//     fn register(&self) -> Result<(), &'static str>;
// }
