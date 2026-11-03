// pub mod connection;
pub mod data_sources;
pub mod node;
pub mod plugin;
pub mod window;

use std::sync::Arc;
use std::sync::mpsc::Sender;
use thiserror::Error;

// use crate::connection::ConnectionData;
use crate::data_sources::DataSource;
use crate::plugin::{Plugin, PluginManager};
use crate::window::MainWindow;

#[derive(Error, Debug)]
pub enum SiloError {
    #[error("data source file does not exist")]
    DataSourceFileMissing,
    #[error("unable to create connection")]
    ConnectionCreationError,
    #[error("plugin not found")]
    PluginNotFoundError,
}

#[derive(Debug)]
pub enum PasswordDialogResult {
    Ok(String),
    Cancel,
}

#[async_trait::async_trait]
pub trait Silo: std::fmt::Debug + Send + Sync {
    fn initialize(&self);
    fn set_workspace_path(&self, path: &str);
    // fn set_window(&self, window: Arc<dyn MainWindow>);
    // fn get_connection_from_name(&self, name: &str) -> Result<Box<dyn Connection>, SiloError>;
    // fn connections(&self) -> Vec<ConnectionData>;
    fn sources(&self) -> Vec<DataSource>;
    fn plugin_manager(&self) -> Arc<dyn PluginManager>;
    // fn get_plugin(&self, plugin: &str) -> Arc<dyn Plugin>;
    fn show_password_dialog(&self, sender: Sender<PasswordDialogResult>) -> String;
}
